//! Koleksi query yang digunakan untuk operasi pada rest API Analytic
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    // dao::AnalyticDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    ID,
};

#[derive(Deserialize, Validate)]
pub struct AreaQuery {
    pub province: String,
    pub city: String,
    pub query: Option<String>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
}

/// Holder untuk implementasi API endpoint publik untuk Analytic.
pub struct PublicApi;

#[api_group("Analytic", "public", base = "/analytic/v1")]
impl PublicApi {
    /// Search for area
    #[api_endpoint(path = "/area", auth = "required")]
    pub fn search_area(query: AreaQuery) -> ApiResult<EntriesResult<()>> {
        query.validate()?;
        // let conn = state.db();
        // let dao = SubReportDao::new(&conn);

        // let sresult = dao.area_search(
        //     &query.query.unwrap_or("".to_string()),
        //     &query.province,
        //     &query.city,
        //     query.offset,
        //     query.limit,
        // )?;

        // let entries = sresult.entries.into_iter().map(|p| p.into()).collect();

        // let count = sresult.count;
        // Ok(ApiResult::success(EntriesResult { count, entries }))
        unimplemented!();
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Analytic", "private", base = "/analytic/v1")]
impl PrivateApi {}
