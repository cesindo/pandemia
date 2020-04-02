//! Core implementasi untuk Service authentikasi.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{types::IdQuery, ApiResult, Error as ApiError, ErrorCode},
    auth::AuthDao,
    dao::AdminDao,
    models,
    prelude::*,
    types::AccountKind,
    user_dao::UserDao,
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

#[derive(Serialize, Deserialize)]
pub struct AccessTokenQuery {
    pub token: String,
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
    /// Meng-otorisasi akun yang telah teregister
    /// User bisa melakukan otorisasi menggunakan email / nomor telp.
    #[api_endpoint(path = "/authorize", auth = "none", mutable)]
    pub fn authorize(state: &mut AppState, query: Authorize) -> ApiResult<AccessToken> {
        let conn = state.db();
        let user = {
            let dao = UserDao::new(&conn);
            if let Some(email) = query.email {
                dao.get_by_email(&email)?
            } else if let Some(phone) = query.phone {
                dao.get_by_phone_num(&phone)?
            } else {
                Err(ApiError::InvalidParameter(
                    ErrorCode::NoLoginInfo as i32,
                    "No email/phone parameter".to_string(),
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

        let user_passhash = dao.get_passhash(AccountKind::User, user.id)?;
        if !crypto::password_match(&query.password, &user_passhash) {
            warn!("user `{}` try to authorize using wrong password", &user.id);
            Err(ApiError::Unauthorized)?
        }

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
    pub fn admin_authorize(state: &mut AppState, query: Authorize) -> ApiResult<AccessToken> {
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

        dao.generate_admin_access_token(user.id)
            .map_err(From::from)
            .map(ApiResult::success)
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

    /// Mendapatkan keypair dari user.
    #[api_endpoint(path = "/get_key", auth = "required")]
    fn user_get_key(query: ()) -> ApiResult<JsonValue> {
        let conn = state.db();
        let dao = UserDao::new(&conn);
        let user_key = dao.get_user_key(current_user.id)?;

        Ok(ApiResult::success(
            json!({"pub_key": user_key.pub_key, "secret_key": user_key.secret_key}),
        ))
    }
}
