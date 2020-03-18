//! Koleksi query yang digunakan untuk operasi pada rest API Pandemia
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    ID,
};

/// New Pandemia query
#[derive(Serialize, Deserialize)]
pub struct NewPandemia {
    pub name: String,
}

/// Holder untuk implementasi API endpoint publik untuk Pandemia.
pub struct PublicApi;

#[api_group("Pandemia", "public", base = "/pandemia/v1")]
impl PublicApi {
    /// Get location info.
    #[api_endpoint(path = "/info_location", auth = "optional")]
    pub fn get_info_location(query: LocationQuery) -> ApiResult<LocationInfoResult> {
        let conn = state.db();
        //@TODO(*): code here
        unimplemented!();
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Pandemia", "private", base = "/pandemia/v1")]
impl PrivateApi {}
