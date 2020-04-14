//! Koleksi query yang digunakan untuk operasi pada rest API Analytic
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    dao::{ReportNoteDao, VillageDataDao},
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

/// Holder untuk implementasi API endpoint publik untuk Analytic.
pub struct PublicApi;

#[api_group("Analytic", "public", base = "/analytic/v1")]
impl PublicApi {
    /// Search for area
    #[api_endpoint(path = "/area", auth = "none")]
    pub fn search_area(query: AreaQuery) -> ApiResult<EntriesResult<VillageData>> {
        query.validate()?;
        let conn = state.db();

        // get area code from province & city
        let area_code: String = get_area_code(&normalize(&query.province), &normalize(&query.city), &conn)?;

        let result = VillageDataDao::new(&conn).list(&area_code, query.offset, query.limit)?;
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

        let area_code: String = get_area_code(&normalize(&query.province), &normalize(&query.city), &conn)?;

        let sresult = dao.search(
            &area_code,
            &query.query.unwrap_or("".to_string()),
            vec![":reviewed:"],
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

// fn title_case(s: &str) -> String {
//     s.split_whitespace()
//         .map(|w| w.chars())
//         .map(|mut c| {
//             c.next()
//                 .into_iter()
//                 .flat_map(|c| c.to_uppercase())
//                 .chain(c.flat_map(|c| c.to_lowercase()))
//         })
//         .map(|c| c.collect::<String>())
//         .collect::<Vec<String>>()
//         .join(" ")
// }

fn normalize(name: &str) -> String {
    // let re = Regex::new("[^a-zA-Z0-9]").unwrap();
    // re.replace_all(name, "-").to_lowercase()
    name.replace("-", " ")
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Analytic", "private", base = "/analytic/v1")]
impl PrivateApi {}
