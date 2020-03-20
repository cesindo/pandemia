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
    dao::RecordDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    ID,
};

/// Holder untuk implementasi API endpoint publik untuk Pandemia.
pub struct PublicApi;

#[api_group("Pandemia", "public", base = "/pandemia/v1")]
impl PublicApi {
    /// Get location info.
    #[api_endpoint(path = "/info_location", auth = "none")]
    pub fn get_info_location(query: LocationQuery) -> ApiResult<Option<models::Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        let mut rec = dao.get_latest_records(query.loc.as_ref().map(|a| a.as_str()), 0, 5)?;

        if rec.first().is_some() {
            Ok(ApiResult::success(rec.pop()))
        } else {
            Ok(ApiResult::success(None))
        }
    }

    // /// Search for records
    // #[api_endpoint(path = "/latest_records", auth = "none")]
    // pub fn latest_records(query: QueryEntries) -> ApiResult<EntriesResult<models::Record>> {
    //     let conn = state.db();
    //     let dao = RecordDao::new(&conn);
    //     let entries = dao
    //         .get_latest_records(query.query.as_ref().map(|a| a.as_str()), query.offset, query.limit)?
    //         .into_iter()
    //         .map(|p| p.into())
    //         .collect();
    //     let count = dao.count()?;
    //     Ok(ApiResult::success(EntriesResult { count, entries }))
    // }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Pandemia", "private", base = "/pandemia/v1")]
impl PrivateApi {}
