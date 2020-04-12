//! Core implementasi untuk Service authentikasi.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{types::IdQuery, ApiResult, Error as ApiError, ErrorCode},
    auth::AuthDao,
    dao::AdminDao,
    models,
    prelude::*,
    types::AccountKind,
    user_dao::{NewUser, NewUserConnect, UserDao},
    util,
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
    pub fn admin_authorize(
        state: &mut AppState,
        query: Authorize,
    ) -> ApiResult<AuthorizeResult<models::Admin>> {
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

        Ok(ApiResult::success(AuthorizeResult {
            access_token,
            user: Some(user),
        }))
    }

    /// Unauthorize current user session, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/admin/unauthorize", auth = "optional", mutable, accessor = "admin")]
    pub fn admin_unauthorize(query: ()) -> ApiResult<()> {
        match current_admin {
            Some(current_admin) => {
                PrivateApi::admin_unauthorize(state, IdQuery { id: current_admin.id }, req)
            }
            None => Ok(ApiResult::success(())),
        }
    }
}
