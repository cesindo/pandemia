//! Koleksi query yang digunakan untuk operasi pada rest API District
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
    dao::{CityDao, DistrictDao},
    error::{Error, ErrorCode},
    models,
    prelude::*,
    ID,
};

/// New District query
#[derive(Serialize, Deserialize, Validate)]
pub struct NewDistrict {
    #[validate(length(max = 100))]
    pub name: String,
    #[validate(length(max = 100))]
    pub city_name: String,
    #[validate(length(max = 100))]
    pub province: String,

    /// Hanya untuk penampungan, bukan district yg sebenarnya
    /// biasanya digunakan untuk mendata dari luar daerah
    pub collector_only: bool,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Holder untuk implementasi API endpoint publik untuk District.
pub struct PublicApi;

#[api_group("District", "public", base = "/district/v1", accessor = "admin")]
impl PublicApi {
    /// Rest API endpoint untuk menambahkan district baru.
    #[api_endpoint(path = "/add", mutable, auth = "required")]
    pub fn add_district(query: NewDistrict) -> ApiResult<models::District> {
        query.validate()?;

        let conn = state.db();

        if !current_admin.has_access("districts") {
            return unauthorized();
        }

        let dao = DistrictDao::new(&conn);

        let city = CityDao::new(&conn).get_by_name(&query.province, &query.city_name)?;

        let city_name = format!("city={}", city.name);
        let province = format!("province={}", city.province);

        let mut meta = vec![city_name, province];

        if query.collector_only {
            meta.push(":collector_only:".to_string());
        }

        if let Some(latitude) = query.latitude {
            meta.push(format!("latitude={}", latitude));
        }
        if let Some(longitude) = query.longitude {
            meta.push(format!("longitude={}", longitude));
        }

        dao.create(&query.name, city.id, &meta.iter().map(|a| a.as_str()).collect())
            .map_err(From::from)
            .map(ApiResult::success)
    }

    /// Mendapatkan daftar district
    #[api_endpoint(path = "/list", auth = "required", accessor = "admin")]
    pub fn list_district(query: QueryEntries) -> ApiResult<EntriesResult<District>> {
        let conn = state.db();
        let dao = DistrictDao::new(&conn);

        let entries = dao.get_districts(query.offset, query.limit)?;

        let count = dao.count()?;
        Ok(ApiResult::success(EntriesResult {
            count,
            entries: entries.into_iter().map(|a| a.into()).collect(),
        }))
    }

    /// Search for districts
    #[api_endpoint(path = "/search", auth = "required", accessor = "admin")]
    pub fn search_districts(query: QueryEntries) -> ApiResult<EntriesResult<District>> {
        query.validate()?;
        let conn = state.db();
        let dao = DistrictDao::new(&conn);

        let sresult = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        let entries = sresult.entries.into_iter().map(|p| p.into()).collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mendapatkan jumlah district secara keseluruhan.
    #[api_endpoint(path = "/count", auth = "required")]
    pub fn district_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let conn = state.db();
        let dao = DistrictDao::new(&conn);

        dao.count().map(ApiResult::success).map_err(From::from)
    }

    /// Mendapatkan data district berdasarkan ID.
    #[api_endpoint(path = "/detail", auth = "required")]
    pub fn district_detail(query: IdQuery) -> ApiResult<models::District> {
        let conn = state.db();
        let dao = DistrictDao::new(&conn);

        dao.get_by_id(query.id)
            .map(ApiResult::success)
            .map_err(From::from)
    }

    /// Delete district.
    #[api_endpoint(path = "/delete", auth = "required", mutable = "true")]
    pub fn delete_district(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = DistrictDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("District", "private", base = "/district/v1")]
impl PrivateApi {}
