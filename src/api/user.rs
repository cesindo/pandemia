//! Koleksi query yang digunakan untuk operasi pada rest API.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    error::{Error, ErrorCode},
    prelude::*,
};

/// Definisi query untuk mendaftarkan akun baru via rest API.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
}

/// Definisi query untuk mengaktifkan akun yang telah didaftarkan.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateUser {
    pub token: String,
    pub password: String,
}

/// Model untuk keperluan tukar menukar data API
/// bukan yang di database (crate::models).
pub mod types {

    use chrono::NaiveDateTime;

    use crate::{api::ApiResult, models};

    use std::convert::From;

    /// Bentuk model akun di dalam database.
    #[derive(Clone, Serialize, Deserialize, PartialEq)]
    pub struct User {
        /// ID dari akun.
        pub id: i64,

        /// Nama lengkap akun.
        pub full_name: String,

        /// Alamat email kun.
        pub email: String,

        /// Nomor telpon akun.
        pub phone_num: String,

        /// Waktu kapan akun ini didaftarkan.
        pub register_time: NaiveDateTime,
    }

    impl From<models::User> for User {
        fn from(a: models::User) -> Self {
            User {
                id: a.id,
                full_name: a.full_name,
                email: a.email,
                phone_num: a.phone_num,
                register_time: a.register_time,
            }
        }
    }

    impl From<models::User> for ApiResult<User> {
        fn from(a: models::User) -> Self {
            ApiResult::success(a.into())
        }
    }
}

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
    pub verif_new_password: String,
}

use crate::models::AccessToken;

/// Holder untuk implementasi API endpoint publik untuk user.
pub struct PublicApi;

#[api_group("User", "public", base = "/user/v1")]
impl PublicApi {
    /// Rest API endpoint untuk mendaftarkan akun baru.
    /// Setelah register akun tidak langsung aktif, perlu melakukan
    /// aktifasi menggunakan endpoint `/user/activate`.
    #[api_endpoint(path = "/user/register", mutable, auth = "none")]
    pub fn register_user(query: RegisterUser) -> ApiResult<String> {
        let conn = state.db();
        let schema = UserDao::new(&conn);

        schema
            .register_user(&query.full_name, &query.email, &query.phone_num)
            .map_err(From::from)
            .map(ApiResult::success)
    }

    /// Mengaktifkan user yang telah teregister.
    /// Ini nantinya dijadikan link yang akan dikirimkan ke email pendaftar.
    #[api_endpoint(path = "/user/activate", auth = "none", mutable)]
    pub fn activate_user(query: ActivateUser) -> ApiResult<types::User> {
        let conn = state.db();
        let schema = UserDao::new(&conn);
        let user = schema.activate_registered_user(query.token)?;
        schema.set_password(user.id, &query.password)?;
        Ok(user.into())
    }

    /// Mendapatkan informasi current user.
    #[api_endpoint(path = "/me/info", auth = "required")]
    pub fn me_info(state: &AppState, query: (), req: &ApiHttpRequest) -> ApiResult<types::User> {
        Ok(ApiResult::success(current_user.into()))
    }

    /// Update password.
    #[api_endpoint(path = "/update_password", auth = "required", mutable)]
    pub fn update_password(query: UpdatePassword) -> ApiResult<()> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        if query.new_password.len() < 6 {
            param_error("New password too short, please use min 6 characters long")?;
        }

        if query.new_password != query.verif_new_password {
            param_error("Password verification didn't match")?;
        }

        let auth_dao = auth::AuthDao::new(&conn);

        let user_passhash = auth_dao.get_passhash("user", current_user.id)?;
        if !crypto::password_match(&query.old_password, &user_passhash) {
            warn!(
                "user `{}` try to update password using wrong password",
                &current_user.id
            );
            Err(ApiError::Unauthorized)?
        }

        dao.set_password(current_user.id, &query.new_password)?;

        Ok(ApiResult::success(()))
    }


    /// Register and connect current account to event push notif (FCM).
    /// Parameter `app_id` adalah app id dari client app.
    #[api_endpoint(path = "/me/connect/create", auth = "none", mutable)]
    pub fn connect_create(query: UserConnect) -> ApiResult<()> {
        query.validate()?;

        let conn = state.db();
        let dao = UserDao::new(&conn);

        dao.create_user_connect(&query.device_id, &query.provider_name, &query.app_id)?;
        Ok(ApiResult::success(()))
    }

    /// Revoke or disconnect current account to event push notif (FCM).
    /// Parameter `app_id` adalah app id dari client app.
    #[api_endpoint(path = "/me/connect/remove", auth = "none", mutable)]
    pub fn connect_remove(query: UserConnect) -> ApiResult<()> {
        query.validate()?;

        let conn = state.db();
        let dao = UserDao::new(&conn);

        dao.remove_user_connect(&query.device_id, &query.provider_name, &query.app_id)?;
        Ok(ApiResult::success(()))
    }

}

use crate::models as db;

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("User", "private", base = "/user/v1")]
impl PrivateApi {
    /// Listing user
    #[api_endpoint(path = "/users", auth = "none")]
    pub fn list_user(query: QueryEntries) -> ApiResult<EntriesResult<db::User>> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        let entries = dao.get_users(query.offset, query.limit)?;

        let count = dao.count()?;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mencari akun berdasarkan kata kunci.
    #[api_endpoint(path = "/search", auth = "none")]
    pub fn search_users(query: QueryEntries) -> ApiResult<EntriesResult<db::User>> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        if query.query.is_none() {
            return Self::list_user(&state, query, req);
        }

        let keyword = query.query.unwrap();

        let (entries, count) = dao.search(&keyword, query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mendapatkan jumlah akun secara keseluruhan.
    #[api_endpoint(path = "/user/count")]
    pub fn user_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        dao.count().map(ApiResult::success).map_err(From::from)
    }

    /// Mendapatkan data akun.
    #[api_endpoint(path = "/user/info", auth = "required")]
    pub fn user_info(query: IdQuery) -> ApiResult<db::User> {
        let conn = state.db();
        let dao = UserDao::new(&conn);

        dao.get_by_id(query.id)
            .map(ApiResult::success)
            .map_err(From::from)
    }
}
