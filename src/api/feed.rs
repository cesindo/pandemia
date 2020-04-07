//! Koleksi query yang digunakan untuk operasi pada rest API Feed
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
    dao::FeedDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    ID,
};

// /// New Feed query
// #[derive(Serialize, Deserialize)]
// pub struct NewFeed {
//     pub loc: String,
// }

#[derive(Deserialize, Validate)]
pub struct FeedQuery {
    #[validate(length(min = 0, max = 500))]
    pub loc: Option<String>,
    #[validate(length(min = 0, max = 500))]
    pub query: Option<String>,
    #[validate(length(min = 0, max = 500))]
    pub exclude_loc: Option<String>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
}

/// Holder untuk implementasi API endpoint publik untuk Feed.
pub struct PublicApi;

#[api_group("Feed", "public", base = "/feed/v1")]
impl PublicApi {
    /// Mendapatkan daftar feed terbaru.
    #[api_endpoint(path = "/query", auth = "none")]
    pub fn query_feed(query: FeedQuery) -> ApiResult<EntriesResult<models::Feed>> {
        query.validate()?;
        let conn = state.db();
        let dao = FeedDao::new(&conn);

        let entries = dao.search(
            query.loc.as_ref().map(|a| a.as_str()),
            query.exclude_loc.as_ref().map(|a| a.as_str()),
            query.offset,
            query.limit,
        )?;
        Ok(ApiResult::success(EntriesResult {
            count: entries.len() as i64,
            entries,
        }))
    }

    /// Mendapatkan jumlah feed secara keseluruhan.
    #[api_endpoint(path = "/count", auth = "required")]
    pub fn feed_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let conn = state.db();
        let dao = FeedDao::new(&conn);

        dao.count().map(ApiResult::success).map_err(From::from)
    }

    /// Mendapatkan data feed berdasarkan ID.
    #[api_endpoint(path = "/detail", auth = "required")]
    pub fn feed_detail(query: IdQuery) -> ApiResult<models::Feed> {
        let conn = state.db();
        let dao = FeedDao::new(&conn);

        dao.get_by_id(query.id)
            .map(ApiResult::success)
            .map_err(From::from)
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Feed", "private", base = "/feed/v1")]
impl PrivateApi {
    // /// Rest API endpoint untuk menambahkan feed baru.
    // #[api_endpoint(path = "/add", mutable, auth = "required")]
    // pub fn add_feed(query: NewFeed) -> ApiResult<models::Feed> {
    //     let conn = state.db();
    //     let dao = FeedDao::new(&conn);

    //     // @TODO(*): Add parameter checking here

    //     dao.create(&query.name)
    //         .map_err(From::from)
    //         .map(ApiResult::success)
    // }

    /// Delete feed.
    #[api_endpoint(path = "/delete", auth = "required", mutable = "true")]
    pub fn delete_feed(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = FeedDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }
}
