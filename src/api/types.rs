//! API message types
//!
#![doc(hidden)]

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api,
    error::{Error, ErrorCode},
    prelude::*,
    ID,
};

pub trait ToApiType<T> {
    // Convert db model into api type
    // updated:
    // menambahkan parameter user untuk mengetahui status
    // apakah user sudah menyukai feed/comment
    fn to_api_type(&self, conn: &PgConnection) -> T;
    // fn to_api_type2(&self, params: i32, conn: &PgConnection) -> T {
    //     self.to_api_type(conn)
    // }
}

#[derive(Serialize, Deserialize)]
pub struct EntriesResult<T> {
    pub entries: Vec<T>,
    pub count: i64,
}

#[derive(Deserialize, Validate)]
pub struct QueryEntries {
    pub query: Option<String>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IdQuery {
    pub id: ID,
}

#[derive(Deserialize, Validate)]
pub struct ResetPassword {
    #[validate(email(message = "Email not valid, please enter valid email address"))]
    pub email: String,
    pub code: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LocationQuery {
    #[validate(length(min = 1, max = 100))]
    pub loc: Option<String>,
}

#[derive(Serialize, Validate)]
pub struct LocationInfoResult {
    pub name: String,
    pub odp: i32,
    pub pdp: i32,
    pub positive: i32,
    pub death: i32,
    pub recovered: i32,
}

#[derive(Deserialize, Validate)]
pub struct UserConnect {
    #[validate(length(min = 1, message = "Device id can't be empty"))]
    pub device_id: String,
    #[validate(length(min = 1, message = "Client app id can't be empty"))]
    pub app_id: String,
    #[validate(length(min = 1, message = "Provider name must be set, eg: android, apple"))]
    pub provider_name: String,
}
