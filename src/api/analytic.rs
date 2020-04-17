//! Koleksi query yang digunakan untuk operasi pada rest API Analytic
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use diesel::dsl::{sql, sum};
use diesel::prelude::*;
use diesel::{sql_query, sql_types::BigInt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    dao::{CityDao, DistrictDataDao, ReportNoteDao, VillageDataDao},
    // dao::AnalyticDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    sqlutil::lower,
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

#[derive(Deserialize, Validate)]
pub struct QueryReportNotes {
    pub province: String,
    pub city: String,
    pub query: Option<String>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
}

#[derive(Deserialize, Validate)]
pub struct GetTotal {
    pub province: String,
    pub city: String,
}

#[derive(Serialize)]
pub struct TotalResult {
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
}

#[derive(QueryableByName)]
struct Entry {
    #[sql_type = "BigInt"]
    odp: i64,
    #[sql_type = "BigInt"]
    pdp: i64,
    #[sql_type = "BigInt"]
    positive: i64,
    #[sql_type = "BigInt"]
    recovered: i64,
    #[sql_type = "BigInt"]
    deaths: i64,
}

/// Holder untuk implementasi API endpoint publik untuk Analytic.
pub struct PublicApi;

#[api_group("Analytic", "public", base = "/analytic/v1")]
impl PublicApi {
    /// Search for area
    #[api_endpoint(path = "/area", auth = "none")]
    pub fn search_area(query: AreaQuery) -> ApiResult<EntriesResult<VillageData>> {
        query.validate()?;
        let conn = state.db();

        // // get area code from province & city
        // let area_code: String = get_area_code(&normalize(&query.province), &normalize(&query.city), &conn)?;
        // let city = CityDao::new(&conn)
        //     .get_by_area_code(&area_code)?
        //     .ok_or(Error::NotFound("City not found by area code".to_string()))?;

        let city = get_city(&query.province, &query.city, &conn)?;

        let result = VillageDataDao::new(&conn).list(city.id, query.offset, query.limit)?;
        Ok(ApiResult::success(EntriesResult {
            count: result.count,
            entries: result.entries.into_iter().map(|a| a.into()).collect(),
        }))
    }

    /// Search for report_notes
    #[api_endpoint(path = "/report_notes", auth = "none")]
    pub fn list_report_notes(query: QueryReportNotes) -> ApiResult<EntriesResult<ReportNote>> {
        query.validate()?;
        let conn = state.db();
        let dao = ReportNoteDao::new(&conn);

        // let area_code: String = get_area_code(&normalize(&query.province), &normalize(&query.city), &conn)?;
        // let city = CityDao::new(&conn)
        //     .get_by_area_code(&area_code)?
        //     .ok_or(Error::NotFound("City not found by area code".to_string()))?;
        let city = get_city(&query.province, &query.city, &conn)?;

        let sresult = dao.search(
            city.id,
            &query.query.unwrap_or("".to_string()),
            "published",
            vec![],
            query.offset,
            query.limit,
        )?;

        let entries = sresult
            .entries
            .into_iter()
            .map(|p| p.to_api_type(&conn))
            .collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Get total data result for specific area.
    #[api_endpoint(path = "/total", auth = "none")]
    pub fn get_total_data(query: GetTotal) -> ApiResult<TotalResult> {
        use crate::schema::village_data::{self, dsl};
        let conn = state.db();
        // let area_code: String = get_area_code(&normalize(&query.province), &normalize(&query.city), &conn)?;

        // let city = CityDao::new(&conn)
        //     .get_by_area_code(&area_code)?
        //     .ok_or(Error::NotFound("City not found by area code".to_string()))?;

        let city = get_city(&query.province, &query.city, &conn)?;

        let mut entry:Vec<Entry> = //village_data::table
            // .filter(dsl::city_id.eq(city_id))
            // .select((
            //     sum(dsl::odp),
            //     sum(dsl::pdp),
            //     sum(dsl::cases),
            //     sum(dsl::recovered),
            //     sum(dsl::deaths),
            // ))
            // .select(sql("SELECT SUM(odp), SUM(pdp), SUM(cases), SUM(recovered), SUM(deaths)"))
            sql_query(&format!("SELECT SUM(odp) as odp, SUM(pdp) as pdp, \
             SUM(cases) as positive, SUM(recovered) as recovered, \
             SUM(deaths) as deaths FROM district_data WHERE city_id={}", city.id))
            .load(&conn)
            .map_err(Error::from)?;

        let result = entry.pop().expect("Cannot get data from db");

        Ok(ApiResult::success(TotalResult {
            odp: result.odp as i32,
            pdp: result.pdp as i32,
            cases: result.positive as i32,
            recovered: result.recovered as i32,
            deaths: result.deaths as i32,
        }))
    }

    /// Get district data.
    #[api_endpoint(path = "/district_data", auth = "none")]
    pub fn get_districts_data(query: AreaQuery) -> ApiResult<EntriesResult<DistrictData>> {
        query.validate()?;
        let conn = state.db();
        let dao = DistrictDataDao::new(&conn);

        let city = get_city(&query.province, &query.city, &conn)?;
        // let city = CityDao::new(&conn)
        //     .get_by_id(&area_code)?
        //     .ok_or(Error::NotFound("City not found by area code".to_string()))?;

        let result = DistrictDataDao::new(&conn).list(city.id, query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult {
            count: result.count,
            entries: result.entries.into_iter().map(|a| a.into()).collect(),
        }))
    }
}

fn get_area_code(prov: &str, city: &str, conn: &PgConnection) -> Result<String> {
    use crate::schema::cities::{self, dsl};
    cities::table
        .filter(
            lower(dsl::province)
                .eq(&normalize(prov))
                .and(lower(dsl::name).eq(&normalize(city))),
        )
        .select(dsl::area_code)
        .first(conn)
        .map_err(Error::from)
}

fn get_city(prov: &str, city: &str, conn: &PgConnection) -> Result<models::City> {
    use crate::schema::cities::{self, dsl};
    cities::table
        .filter(
            lower(dsl::province)
                .eq(&normalize(prov))
                .and(lower(dsl::name).eq(&normalize(city))),
        )
        // .select(dsl::id)
        .first(conn)
        .map_err(Error::from)
}

fn normalize(name: &str) -> String {
    // let re = Regex::new("[^a-zA-Z0-9]").unwrap();
    // re.replace_all(name, "-").to_lowercase()
    name.replace("-", " ")
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Analytic", "private", base = "/analytic/v1")]
impl PrivateApi {}
