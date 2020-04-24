//! Koleksi query yang digunakan untuk operasi pada rest API City
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::*, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    dao::CityDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    util, ID,
};

/// New City query
#[derive(Serialize, Deserialize)]
pub struct NewCity {
    pub name: String,
    pub province: String,
    pub country_code: String,
    pub area_code: String,
}

/// Holder untuk implementasi API endpoint publik untuk city.
pub struct PublicApi;

#[api_group("City", "public", base = "/city/v1", accessor = "admin")]
impl PublicApi {
    /// Rest API endpoint untuk menambahkan city baru.
    #[api_endpoint(path = "/add", mutable, auth = "required")]
    pub fn add_city(query: NewCity) -> ApiResult<models::City> {
        let conn = state.db();
        let dao = CityDao::new(&conn);

        if current_admin.has_access("update_city") {
            return unauthorized();
        }

        // @TODO(*): Add parameter checking here

        dao.create(
            &query.name,
            &query.province,
            &query.country_code,
            &query.area_code,
        )
        .map_err(From::from)
        .map(ApiResult::success)
    }

    // /// Mendapatkan daftar city
    // #[api_endpoint(path = "/list", auth = "required")]
    // pub fn list_city(query: QueryEntries) -> ApiResult<EntriesResult<models::City>> {
    //     let conn = state.db();
    //     let dao = CityDao::new(&conn);

    //     let entries = dao.get_citys(query.offset, query.limit)?;

    //     let count = dao.count()?;
    //     Ok(ApiResult::success(EntriesResult { count, entries }))
    // }

    /// Search for city
    #[api_endpoint(path = "/search", auth = "required")]
    pub fn search_city(query: QueryEntries) -> ApiResult<EntriesResult<models::City>> {
        query.validate()?;
        let conn = state.db();
        let dao = CityDao::new(&conn);

        let sresult = dao.search(
            &util::title_case(&query.query.unwrap_or("".to_string())),
            query.offset,
            query.limit,
        )?;

        let entries = sresult.entries.into_iter().map(|p| p.into()).collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mendapatkan jumlah city secara keseluruhan.
    #[api_endpoint(path = "/count", auth = "required")]
    pub fn city_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let conn = state.db();
        let dao = CityDao::new(&conn);

        dao.count().map(ApiResult::success).map_err(From::from)
    }

    /// Mendapatkan data city berdasarkan ID.
    #[api_endpoint(path = "/detail", auth = "required")]
    pub fn city_detail(query: IdQuery) -> ApiResult<models::City> {
        let conn = state.db();
        let dao = CityDao::new(&conn);

        dao.get_by_id(query.id)
            .map(ApiResult::success)
            .map_err(From::from)
    }

    /// Delete city.
    #[api_endpoint(path = "/delete", auth = "required", mutable = "true")]
    pub fn delete_city(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = CityDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("City", "private", base = "/city/v1")]
impl PrivateApi {}
