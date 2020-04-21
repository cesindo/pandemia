//! Koleksi query yang digunakan untuk operasi pada rest API Analytic
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::dsl::{sql, sum};
use diesel::prelude::*;
use diesel::{
    sql_query,
    sql_types::{BigInt, Date, Text},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    dao::{CityDao, DistrictDao, DistrictDataDao, ReportNoteDao, VillageDataDao},
    // dao::AnalyticDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    sqlutil::lower,
    types::LocKind,
    util,
    ID,
};

use std::collections::HashMap;

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

#[derive(QueryableByName, Serialize, Debug)]
pub struct TravelerData {
    #[sql_type = "Text"]
    pub loc_path: String,

    #[sql_type = "Date"]
    pub updated: NaiveDate,

    #[sql_type = "BigInt"]
    pub ppdwt: i64,

    #[sql_type = "BigInt"]
    pub odp: i64,

    #[sql_type = "BigInt"]
    pub pptb: i64,
}

#[derive(QueryableByName, Serialize, Debug)]
pub struct GeneralData {
    #[sql_type = "Text"]
    pub loc_path: String,

    #[sql_type = "Date"]
    pub updated: NaiveDate,

    #[sql_type = "BigInt"]
    pub odp: i64,

    #[sql_type = "BigInt"]
    pub pdp: i64,

    #[sql_type = "BigInt"]
    pub positive: i64,

    #[sql_type = "BigInt"]
    pub deaths: i64,

    #[sql_type = "BigInt"]
    pub recovered: i64,
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

    /// Get general trends data for time series drawing.
    #[api_endpoint(path = "/trend/general", auth = "none")]
    pub fn get_general_trend_data(query: AreaQuery) -> ApiResult<TrendData> {
        query.validate()?;
        let conn = state.db();

        let city = get_city(&query.province, &query.city, &conn)?;

        let loc_path = format!("/Indonesia/{}/{}", city.province, city.name);

        debug!("trying to get general trend data from loc_path: {}", loc_path);

        let data: Vec<GeneralData> = sql_query(&format!(
            "SELECT loc_path, last_updated::timestamp::date AS updated, \
            SUM(odp) AS odp, \
            SUM(pdp) AS pdp, \
            SUM(total_cases) AS positive, \
            SUM(total_deaths) AS deaths, \
            SUM(total_recovered) AS recovered \
            FROM records WHERE loc_path = '{}' \
            GROUP BY (loc_path, updated) \
            ORDER BY updated ASC \
            LIMIT 30",
            loc_path
        ))
        .load(&conn)
        .map_err(Error::from)?;

        dbg!(&data);

        let mut agg: HashMap<String, Vec<i64>> = HashMap::new();

        // let mut trends = vec![];
        let mut cats = vec![];
        // let mut series = vec![];

        for d in data {
            if !cats.contains(&d.updated.to_string()) {
                cats.push(d.updated.to_string());
            }
            if agg.get("odp").is_some() {
                agg.get_mut("odp").unwrap().push(d.odp);
            } else {
                agg.insert("odp".to_string(), vec![d.odp]);
            }

            if agg.get("pdp").is_some() {
                agg.get_mut("pdp").unwrap().push(d.pdp);
            } else {
                agg.insert("pdp".to_string(), vec![d.pdp]);
            }

            if agg.get("positive").is_some() {
                agg.get_mut("positive").unwrap().push(d.positive);
            } else {
                agg.insert("positive".to_string(), vec![d.positive]);
            }

            if agg.get("deaths").is_some() {
                agg.get_mut("deaths").unwrap().push(d.deaths);
            } else {
                agg.insert("deaths".to_string(), vec![d.deaths]);
            }

            if agg.get("recovered").is_some() {
                agg.get_mut("recovered").unwrap().push(d.recovered);
            } else {
                agg.insert("recovered".to_string(), vec![d.recovered]);
            }
        }

        let rv = TrendData {
            cats,
            series: vec![
                Serie {
                    name: "ODP".to_string(),
                    data: agg.remove("odp").unwrap_or(vec![]),
                },
                Serie {
                    name: "PDP".to_string(),
                    data: agg.remove("pdp").unwrap_or(vec![]),
                },
                Serie {
                    name: "COVID19".to_string(),
                    data: agg.remove("positive").unwrap_or(vec![]),
                },
                Serie {
                    name: "Sembuh".to_string(),
                    data: agg.remove("recovered").unwrap_or(vec![]),
                },
                Serie {
                    name: "Meninggal".to_string(),
                    data: agg.remove("deaths").unwrap_or(vec![]),
                },
            ],
        };

        // debug!("got {} data series", series.len());

        Ok(ApiResult::success(rv))
    }

    /// Get traveler trends data for time series drawing.
    #[api_endpoint(path = "/trend/traveler", auth = "none")]
    pub fn get_traveler_trend_data(query: AreaQuery) -> ApiResult<TrendData> {
        query.validate()?;
        let conn = state.db();

        let city = get_city(&query.province, &query.city, &conn)?;

        let loc_path = format!("/Indonesia/{}/{}", city.province, city.name);

        debug!("trying to get traveler trend data from loc_path: {}", loc_path);

        let data: Vec<TravelerData> = sql_query(&format!(
            "SELECT loc_path, last_updated::timestamp::date AS updated, \
            SUM(ppdwt) AS ppdwt, \
            SUM(odp) AS odp, \
            SUM(pptb) AS pptb \
            FROM records WHERE loc_path = '{}' \
            GROUP BY (loc_path, updated) \
            ORDER BY updated ASC \
            LIMIT 30",
            loc_path
        ))
        .load(&conn)
        .map_err(Error::from)?;

        dbg!(&data);

        let mut agg: HashMap<String, Vec<i64>> = HashMap::new();

        // let mut trends = vec![];
        let mut cats = vec![];
        // let mut series = vec![];

        for d in data {
            if !cats.contains(&d.updated.to_string()) {
                cats.push(d.updated.to_string());
            }
            if agg.get("ppdwt").is_some() {
                agg.get_mut("ppdwt").unwrap().push(d.ppdwt);
            // ppdwt.push(d.ppdwt);
            } else {
                agg.insert("ppdwt".to_string(), vec![d.ppdwt]);
            }

            if agg.get("odp").is_some() {
                agg.get_mut("odp").unwrap().push(d.odp);
            } else {
                agg.insert("odp".to_string(), vec![d.odp]);
            }

            if agg.get("pptb").is_some() {
                agg.get_mut("pptb").unwrap().push(d.pptb);
            } else {
                agg.insert("pptb".to_string(), vec![d.pptb]);
            }
        }

        let rv = TrendData {
            cats,
            series: vec![
                Serie {
                    name: "Pelaku Perjalanan Dari Wilayah Terjangkit".to_string(),
                    data: agg.remove("ppdwt").unwrap_or(vec![]),
                },
                Serie {
                    name: "ODP".to_string(),
                    data: agg.remove("odp").unwrap_or(vec![]),
                },
                Serie {
                    name: "Pelaku Perjalanan Dari Wilayah Terjangkit Tanpa Gejala".to_string(),
                    data: agg.remove("pptb").unwrap_or(vec![]),
                },
            ],
        };

        // debug!("got {} data series", series.len());

        Ok(ApiResult::success(rv))
    }

    /// Get district data.
    #[api_endpoint(path = "/data/location_address", auth = "none")]
    pub fn get_location_address(query: ()) -> ApiResult<Vec<IdAddress>> {
        use crate::schema::cities::{self, dsl};
        let conn = state.db();

        let cities: Vec<(i64, String, String)> = {
            cities::table
                .select((dsl::id, dsl::name, dsl::province))
                .limit(1000)
                .load(&conn)
                .map_err(Error::from)?
        };

        let mut addresses: Vec<IdAddress> = cities
            .iter()
            .map(|a| IdAddress {
                id: a.0,
                name: a.1.to_owned(),
                address: format!("{}, {}", a.1, a.2),
                kind: LocKind::City as i16,
                path: format!("/Indonesia/{}/{}", a.2, a.1),
            })
            .collect();

        for city in cities {
            use crate::schema::districts::{self, dsl};
            let districts: Vec<(i64, String)> = {
                districts::table
                    .filter(dsl::city_id.eq(city.0))
                    .select((dsl::id, dsl::name))
                    .limit(1000)
                    .load(&conn)
                    .map_err(Error::from)?
            };

            for district in districts {
                addresses.push(IdAddress {
                    id: district.0,
                    name: district.1.to_owned(),
                    address: format!("{}, {}, {}", district.1, city.1, city.2),
                    kind: LocKind::District as i16,
                    path: format!("/Indonesia/{}/{}/{}", city.2, city.1, district.1),
                })
            }
        }

        Ok(ApiResult::success(addresses))
    }

    /// Get district data.
    #[api_endpoint(path = "/data/districts", auth = "none")]
    pub fn get_district_data(query: AreaQuery) -> ApiResult<Vec<IdAddress>> {
        use crate::schema::districts::{self, dsl};

        let conn = state.db();
        let dao = DistrictDao::new(&conn);

        let city = get_city(&query.province, &query.city, &conn)?;

        let districts: Vec<(i64, String)> = {
            districts::table
                .filter(dsl::city_id.eq(city.id))
                .select((dsl::id, dsl::name))
                .load(&conn)
                .map_err(Error::from)?
        };

        // let cities: Vec<(i64, String)> = {
        //     use crate::schema::cities::{self, dsl};
        //     cities::table
        //         // .filter(dsl::province.eq( util::title_case(&normalize(&province)) ))
        //         .select((dsl::id, dsl::name, dsl::province))
        //         .load(&conn)
        //         .map_err(Error::from)?
        // };

        Ok(ApiResult::success(
            districts
                .into_iter()
                .map(|a| IdAddress {
                    id: a.0,
                    name: a.1.to_owned(),
                    address: a.1.to_owned(),
                    kind: LocKind::District as i16,
                    path: format!("/Indonesia/{}/{}", city.province, a.1),
                })
                .collect(),
        ))
    }
}

#[derive(Serialize)]
pub struct IdAddress {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub kind: i16,
    pub path: String,
}

#[derive(Serialize)]
pub struct Serie {
    pub name: String,
    pub data: Vec<i64>,
}

#[derive(Serialize)]
pub struct TrendData {
    pub cats: Vec<String>,
    pub series: Vec<Serie>,
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

/// Mendapatkan data city dari url formatted prov dan city
fn get_city(prov: &str, city: &str, conn: &PgConnection) -> Result<models::City> {
    use crate::schema::cities::{self, dsl};
    cities::table
        .filter(
            lower(lower(dsl::province))
                .eq(&normalize(prov).to_lowercase())
                .and(lower(dsl::name).eq(&normalize(city).to_lowercase())),
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
