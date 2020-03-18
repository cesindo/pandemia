//! API message types
//!
#![doc(hidden)]

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api,
    error::{Error, ErrorCode},
    prelude::*,
    ID,
};

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
    pub name: String,
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
