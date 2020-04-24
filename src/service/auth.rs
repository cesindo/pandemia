//! Core implementasi untuk Service authentikasi.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::{Datelike, NaiveDateTime, Timelike};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{error::*, types::*, ApiResult, Error as ApiError, ErrorCode},
    auth::AuthDao,
    dao::{AdminDao, CityDao, Logs},
    error::Error,
    kvstore::KvStore,
    models,
    prelude::*,
    types::AccountKind,
    user_dao::{NewUser, NewUserConnect, UserDao},
    util, ID,
};

/// Core basis service untuk authentikasi.
pub struct AuthService {}

impl AuthService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for AuthService {
    fn name(&self) -> &'static str {
        "auth"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().link(PublicApi::wire);
        builder.private_scope().link(PrivateApi::wire);
    }
}

use crate::models::AccessToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorize {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeviceAuthorize {
    #[validate(length(min = 5, max = 1000))]
    pub device_id: String,
    #[validate(length(min = 5, max = 1000))]
    pub fcm_token: String,
    #[validate(length(min = 3, max = 10))]
    pub platform: String,
    #[validate(length(min = 3, max = 100))]
    pub loc_name: String,
    #[validate(length(min = 3, max = 100))]
    pub loc_name_full: String,
    pub loc_long: f64,
    pub loc_lat: f64,
    pub loc_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenQuery {
    pub token: String,
}

#[derive(Serialize, Validate)]
pub struct AuthorizeResult<T> {
    pub access_token: models::AccessToken,
    pub user: Option<T>,
}

struct PrivateApi;

#[api_group("Authorization", "private", base = "/auth/v1")]
impl PrivateApi {
    /// Menghapus akses token
    #[api_endpoint(path = "/remove_access_token", auth = "required", mutable)]
    pub fn remove_access_token(query: AccessTokenQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = AuthDao::new(&conn);
        dao.remove_access_token(&query.token)?;

        Ok(ApiResult::success(()))
    }

    /// Unauthorize user, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/unauthorize", auth = "required", mutable)]
    pub fn unauthorize(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = AuthDao::new(&conn);

        dao.clear_access_token_by_user_id(query.id)?;

        Ok(ApiResult::success(()))
    }

    /// Unauthorize user, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/admin/unauthorize", auth = "required", mutable)]
    pub fn admin_unauthorize(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = AuthDao::new(&conn);

        dao.clear_access_token_by_admin_id(query.id)?;

        Ok(ApiResult::success(()))
    }
}

struct PublicApi;

/// API endpoint untuk keperluan otorisasi.
#[api_group("Authorization", "public", base = "/auth/v1")]
impl PublicApi {
    /// Authorize user's device.
    /// Ini akan otomatis membuat user baru dan menghubungkan token push notif provider
    /// ke user tersebut.
    #[api_endpoint(path = "/device/authorize", auth = "none", mutable)]
    pub fn authorize_device(query: DeviceAuthorize) -> ApiResult<AccessToken> {
        query.validate()?;

        let conn = state.db();

        let dao = UserDao::new(&conn);

        let kv = KvStore::new(&conn);

        let user_device_key = format!("user-device.{}", query.device_id);

        // check apabila sudah terdaftar dan kemungkinan akses token expired
        // generasikan akses token baru untuk device yang telah terdaftar
        if let Ok(user_id) = kv.get(&user_device_key) {
            if let Some(user_id) = user_id {
                let (user_id, full_name) = {
                    let s: Vec<&str> = user_id.split("|").collect();
                    (s[0].to_string(), s[1].to_string())
                };
                if let Ok(user_id) = user_id.parse::<i64>() {
                    let user = dao.get_by_id(user_id)?;

                    let token = AuthDao::new(&conn).generate_access_token(user.id)?;
                    // .map_err(From::from)?;
                    // .map(ApiResult::success);

                    dao.update_user_location(
                        &user,
                        &query.device_id,
                        &query.loc_name,
                        &query.loc_path.unwrap_or(query.loc_name_full),
                    )?;

                    return Ok(ApiResult::success(token));
                }
            }
        }

        // gunakan semuanya random hanya untuk memudahkan push notif saja
        let gen_name = format!("gen__{}_{}", util::random_string(20), util::random_number());

        let (user, (_, _)) = dao.create_user(
            &NewUser {
                full_name: &gen_name,
                email: &format!("{}@pandemia.net", gen_name),
                phone_num: &format!(
                    "085{}{}{}{}{}",
                    util::random_number(),
                    util::random_number(),
                    util::random_number(),
                    util::random_number(),
                    util::current_time_millis()
                ),
                active: true,
                register_time: util::now(),
            },
            Some(NewUserConnect {
                user_id: 0,
                device_id: &query.device_id,
                provider_name: &query.platform,
                app_id: &query.fcm_token,
                latest_loc: &query.loc_name,
                latest_loc_full: &query.loc_name_full,
                latest_loc_long: query.loc_long,
                latest_loc_lat: query.loc_lat,
            }),
        )?;

        dao.update_user_location(
            &user,
            &query.device_id,
            &query.loc_name,
            &query.loc_path.unwrap_or(query.loc_name_full),
        )?;

        kv.set(&user_device_key, &format!("{}|{}", user.id, user.full_name))?;

        // set default settings
        let _ = user.set_setting("enable_push_notif", "true", &conn);

        let dao = AuthDao::new(&conn);

        dao.generate_access_token(user.id)
            .map_err(From::from)
            .map(ApiResult::success)
    }

    /// Unauthorize current user session, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/unauthorize", auth = "optional", mutable)]
    pub fn unauthorize(query: ()) -> ApiResult<()> {
        match current_user {
            Some(current_user) => PrivateApi::unauthorize(state, IdQuery { id: current_user.id }, req),
            None => Ok(ApiResult::success(())),
        }
    }

    /// Meng-otorisasi akun admin
    /// Admin bisa melakukan otorisasi menggunakan email / nomor telp.
    #[api_endpoint(path = "/admin/authorize", auth = "none", mutable)]
    pub fn admin_authorize(state: &mut AppState, query: Authorize) -> ApiResult<AuthorizeResult<Admin>> {
        let conn = state.db();
        let user = {
            let dao = AdminDao::new(&conn);
            if let Some(email) = query.email {
                dao.get_by_email(&email)?
            } else {
                Err(ApiError::InvalidParameter(
                    ErrorCode::NoLoginInfo as i32,
                    "No email parameter".to_string(),
                ))?
            }
        };

        if !user.active {
            return Err(ApiError::InvalidParameter(
                ErrorCode::Unauthorized as i32,
                "Account blocked".to_string(),
            ));
        }

        let dao = AuthDao::new(&conn);

        let user_passhash = dao.get_passhash(AccountKind::Admin, user.id)?;
        if !crypto::password_match(&query.password, &user_passhash) {
            warn!("user `{}` try to authorize using wrong password", &user.id);
            Err(ApiError::Unauthorized)?
        }

        let access_token = dao.generate_admin_access_token(user.id)?;

        Logs::new(&conn).write(&format!("{} logged in", user.name), user.id);

        Ok(ApiResult::success(AuthorizeResult {
            access_token,
            user: Some(user.to_api_type(&conn)),
        }))
    }

    /// Unauthorize current user session, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/admin/unauthorize", auth = "optional", mutable, accessor = "admin")]
    pub fn admin_unauthorize(query: ()) -> ApiResult<()> {
        match current_admin {
            Some(current_admin) => {
                let conn = state.db();

                let rv = PrivateApi::admin_unauthorize(state, IdQuery { id: current_admin.id }, req);

                Logs::new(&conn).write(&format!("{} logged out", current_admin.name), current_admin.id);

                rv
            }
            None => Ok(ApiResult::success(())),
        }
    }

    /// Authorize satgas.
    #[api_endpoint(path = "/satgas/authorize", auth = "none", mutable)]
    pub fn satgas_authorize(query: SatgasAuthorize) -> ApiResult<SatgasAuthorizeResult> {
        let conn = state.db();

        let kv = KvStore::new(&conn);

        let entry_key = format!("web-token.{}", query.token.trim());

        let user_id = kv
            .get(&entry_key)?
            .ok_or(ApiError::NotFound(404, "Web token tidak valid".to_string()))?
            .parse::<i64>()?;

        let user_dao = UserDao::new(&conn);

        let user = user_dao.get_by_id(user_id)?;

        if !user.is_satgas() || user.is_blocked() || user.is_deleted() {
            return Err(ApiError::Unauthorized);
        }

        // let token = generate_web_token(&user, &conn)?;

        // if token != query.token {
        //     return Err(ApiError::Unauthorized);
        // }

        // delete entry
        kv.delete(&entry_key)?;

        AuthDao::new(&conn)
            .generate_access_token(user.id)
            .map_err(From::from)
            .map(|at| {
                ApiResult::success(SatgasAuthorizeResult {
                    token: at.token,
                    user: user.to_api_type(&conn),
                })
            })
    }

    /// Generate web token for login.
    #[api_endpoint(path = "/satgas/get_web_token", auth = "required", mutable, accessor = "user")]
    pub fn get_web_token(query: IdQuery) -> ApiResult<String> {
        let conn = state.db();

        if !current_user.is_satgas() || current_user.is_blocked() || current_user.is_deleted() {
            return Err(ApiError::Unauthorized);
        }

        let kv = KvStore::new(&conn);

        // clear up token lama
        kv.delete_by_values(&format!("{}", current_user.id))?;

        let token = generate_web_token(&current_user, &conn)?;

        kv.set(&format!("web-token.{}", token), &format!("{}", current_user.id))?;

        Ok(ApiResult::success(token))
    }

    /// Get city area code.
    #[api_endpoint(path = "/get_area_code", auth = "required", accessor = "admin")]
    pub fn get_area_code(query: IdQuery) -> ApiResult<String> {
        let conn = state.db();

        if query.id == 0 || query.id != current_admin.get_city_id().unwrap_or(0) {
            return unauthorized();
        }

        let dao = CityDao::new(&conn);
        let city = dao.get_by_id(query.id)?;

        Ok(ApiResult::success(city.area_code))
    }

    /// reset area code.
    #[api_endpoint(path = "/reset_area_code", auth = "required", mutable, accessor = "admin")]
    pub fn reset_area_code(query: IdQuery) -> ApiResult<String> {
        use crate::schema::cities::{self, dsl};

        let conn = state.db();

        if !current_admin.has_access("reset_area_code") && current_admin.get_city_id() != Some(query.id) {
            return unauthorized();
        }

        let city = CityDao::new(&conn).get_by_id(query.id)?;

        let area_code = format!(
            "{}{}{}{}{}",
            &city.name[0..1],
            util::random_number(),
            util::random_number(),
            util::random_number(),
            util::random_number()
        );

        let conn = state.db();
        diesel::update(dsl::cities.filter(dsl::id.eq(query.id)))
            .set(dsl::area_code.eq(&area_code))
            .execute(&conn)
            .map_err(Error::from)?;

        Ok(ApiResult::success(area_code))
    }
}

// #[derive(Deserialize, Validate)]
// pub struct ResetAreaCode {
//     pub id: ID,
//     pub area_code: String,
// }

#[derive(Deserialize, Validate)]
pub struct SatgasAuthorize {
    pub token: String,
}

#[derive(Serialize)]
pub struct SatgasAuthorizeResult {
    pub token: String,
    pub user: Satgas,
}

fn generate_web_token(user: &models::User, conn: &PgConnection) -> Result<String> {
    let device_id: String = {
        use crate::schema::user_connect::{self, dsl};
        user_connect::table
            .filter(dsl::user_id.eq(user.id))
            .select(dsl::device_id)
            .first(conn)
            .map_err(Error::from)?
    };

    let now = util::now();

    let toh = format!(
        "{}/{}/{}/{}/{}",
        user.id,
        device_id,
        now.day(),
        now.hour(),
        now.second()
    );

    let hash = crypto::hash_str(&toh).to_hex();

    let token = (&hash[0..6]).to_string().to_uppercase();

    Ok(token)
}
