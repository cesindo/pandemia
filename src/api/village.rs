//! Koleksi query yang digunakan untuk operasi pada rest API Village
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::*, parsed_query::*, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    dao::{CityDao, Logs, VillageDao, VillageDataDao},
    error,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    sub_report_dao,
    types::{HealthyKind, LocKind, Ops, SubReportStatus},
    village_data_dao::{NewVillageData, UpdateVillageData},
    ID,
};

/// New Village query
#[derive(Serialize, Deserialize)]
pub struct NewVillage {
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct AddVillage {
    #[validate(length(min = 2, max = 1000))]
    pub name: String,
    #[validate(length(min = 2, max = 1000))]
    pub district: String,
    #[validate(length(min = 2, max = 1000))]
    pub city: String,
    #[validate(length(min = 2, max = 1000))]
    pub province: String,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Deserialize, Validate)]
pub struct RecordUpdate {
    #[validate(range(min = 1, max = 9999999999))]
    pub id: i64,
    #[validate(range(min = 1))]
    pub village_id: i64,
    #[validate(length(min = 2, max = 1000))]
    pub village_name: String,
    pub district_name: String,
    pub cases: i32,
    pub deaths: i32,
    pub recovered: i32,
    pub ppdwt: i32,
    pub pptb: i32,
    pub odp: i32,
    pub odpsp: i32,
    pub pdp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
    // pub meta: Vec<String>,
    pub last_updated: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct CommitData {
    province: Option<String>,
    city: Option<String>,
    records: Vec<RecordUpdate>,
}

/// Holder untuk implementasi API endpoint publik untuk village.
pub struct PublicApi;

#[api_group("Village", "public", base = "/village/v1")]
impl PublicApi {
    /// Add village.
    #[api_endpoint(path = "/add", auth = "required", mutable, accessor = "admin")]
    pub fn add_village(query: AddVillage) -> ApiResult<models::Village> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let city = CityDao::new(&conn)
            .get_by_name(&query.province, &query.city)
            .map_err(|_| {
                ApiError::BadRequest(83913, format!("Tidak ada kota/kab dengan nama {}", query.city))
            })?;

        let village = dao.create(
            &query.name,
            &query.district,
            &query.city,
            &query.province,
            query.latitude.parse::<f64>()?,
            query.longitude.parse::<f64>()?,
            &vec![],
            city.id,
            0,
        )?;
        Ok(ApiResult::success(village))
    }

    /// Search for villages
    #[api_endpoint(path = "/search", auth = "optional")]
    pub fn search_villages(query: QueryEntries) -> ApiResult<EntriesResult<models::Village>> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let sresult = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        // let entries = sresult.entries.into_iter().map(|p| p.into()).collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult {
            count,
            entries: sresult.entries,
        }))
    }

    /// Delete village.
    #[api_endpoint(path = "/delete", auth = "required", mutable, accessor = "admin")]
    pub fn delete_village(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();

        let dao = VillageDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }

    /// Search for village_data
    #[api_endpoint(path = "/village_data/search", auth = "required", accessor = "admin")]
    pub fn search_village_data(query: QueryEntries) -> ApiResult<EntriesResult<VillageData>> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDataDao::new(&conn);

        let parq = match query.query.as_ref() {
            Some(q) => parse_query(q),
            None => ParsedQuery::default(),
        };

        let sresult = dao.search(
            parq.district_name,
            // parq.village_name,
            query.query.as_ref().unwrap_or(&"".to_string()),
            query.offset,
            query.limit,
        )?;

        let entries = sresult.entries.into_iter().map(|p| p.into()).collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    // /// Rest API endpoint untuk menambahkan village baru.
    // #[api_endpoint(path = "/add", mutable, auth = "required")]
    // pub fn add_village(query: NewVillage) -> ApiResult<models::Village> {
    //     let conn = state.db();
    //     let dao = VillageDao::new(&conn);

    //     // @TODO(*): Add parameter checking here

    //     dao
    //         .create(
    //             &query.name,
    //         )
    //         .map_err(From::from)
    //         .map(ApiResult::success)
    // }

    /// Mendapatkan data village berdasarkan ID.
    #[api_endpoint(path = "/detail", auth = "required")]
    pub fn village_detail(query: IdQuery) -> ApiResult<models::Village> {
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        dao.get_by_id(query.id)
            .map(ApiResult::success)
            .map_err(From::from)
    }

    /// Update multiple records at once.
    #[api_endpoint(path = "/commit", auth = "required", mutable, accessor = "admin")]
    pub fn commit(query: CommitData) -> ApiResult<()> {
        use crate::schema::village_data::{self, dsl};
        query.validate()?;

        let conn = state.db();

        let locs = query
            .records
            .iter()
            .map(|a| format!("'{}/{}'", a.district_name, a.village_name))
            .collect::<Vec<String>>();

        if current_admin.get_city_id().is_none() && current_admin.id != 1 {
            return unauthorized();
        }

        // let city =
        // if current_admin.id != 1 && (query.city.is_some() || query.province.is_some()) {
        //     return param_error(
        //         "Anda tidak bisa menset parameter city / province"
        //     )
        // } else {
        //     match (query.province.as_ref(), query.city.as_ref()){
        //         (Some(province), Some(city)) => {
        //             CityDao::new(&conn).get_by_name(province, city)?
        //         },
        //         _ => return param_error("Province and city parameter not set")
        //     }
        // };

        conn.build_transaction()
            .read_write()
            .run::<_, error::Error, _>(|| {
                let dao = VillageDataDao::new(&conn);

                for record in query.records {
                    let meta: Vec<String> = {
                        village_data::table
                            .filter(dsl::id.eq(record.id))
                            .select(dsl::meta)
                            .first(&conn)?
                    };

                    // // if let Some(ca) = current_admin {
                    // meta.push(":updated_by_admin:".to_string());
                    // meta.push(format!("updated_by_admin_name={}", current_admin.name));
                    // meta.push(format!("updated_by_admin_id={}", current_admin.id));
                    // // }

                    // meta.dedup();

                    dao.update(
                        record.village_id,
                        Ops::Set,
                        &UpdateVillageData {
                            odp: record.odp,
                            pdp: record.pdp,
                            cases: record.cases,
                            recovered: record.recovered,
                            deaths: record.deaths,
                            last_updated_by_id: current_admin.id,
                            meta: &meta.iter().map(|a| a.as_str()).collect(),
                            city_id: None,
                            ppdwt: record.ppdwt,
                            pptb: record.pptb,
                            odpsp: record.odpsp,
                            pdps: record.pdps,
                            pdpm: record.pdpm,
                            otg: record.otg,
                        },
                    )?;

                    debug!("updating village data id {}...", record.id);
                }

                Ok(())
            })?;

        Logs::new(&conn).write(
            &format!(
                "{} update village data for {} data: {}",
                current_admin.name,
                locs.len(),
                locs.join(", ")
            ),
            current_admin.id,
        );

        Ok(ApiResult::success(()))
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Village", "private", base = "/village/v1")]
impl PrivateApi {
    /// Mendapatkan daftar village
    #[api_endpoint(path = "/list", auth = "required")]
    pub fn list_village(query: QueryEntries) -> ApiResult<EntriesResult<models::Village>> {
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let entries = dao.get_villages(query.offset, query.limit)?;

        let count = dao.count()?;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mendapatkan jumlah village secara keseluruhan.
    #[api_endpoint(path = "/count", auth = "required")]
    pub fn village_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        dao.count().map(ApiResult::success).map_err(From::from)
    }

    /// Delete village.
    #[api_endpoint(path = "/delete", auth = "required", mutable = "true")]
    pub fn delete_village(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }
}
