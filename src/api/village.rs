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
    dao::{CityDao, DistrictDataDao, Logs, VillageDao, VillageDataDao},
    district_data_dao::UpdateDistrictData,
    error,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    sub_report_dao,
    types::{HealthyKind, LocKind, Ops, SubReportStatus},
    util,
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
    // pub last_updated: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct AddVillageData {
    pub village_id: i64,
    pub record: RecordUpdate,
}

#[derive(Deserialize, Validate)]
pub struct CommitData {
    province: Option<String>,
    city: Option<String>,
    records: Vec<RecordUpdate>,
}

#[derive(Deserialize, Validate)]
pub struct VillageSearch {
    pub query: Option<String>,
    /// Scope is location path (loc_path), eg: /ID/Jawa Tengah/Wonosobo
    pub scope: Option<String>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
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
    pub fn search_villages(query: VillageSearch) -> ApiResult<EntriesResult<models::Village>> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let sresult = dao.search(
            &query.query.unwrap_or("".to_string()),
            query.scope.as_ref().map(|a| a.as_str()),
            query.offset,
            query.limit,
        )?;

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
    #[api_endpoint(path = "/village_data/search", auth = "required", accessor = "admin,user")]
    pub fn search_village_data(query: QueryEntries) -> ApiResult<EntriesResult<VillageData>> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDataDao::new(&conn);

        let parq = match query.query.as_ref() {
            Some(q) => parse_query(q),
            None => ParsedQuery::default(),
        };

        if let Some(current_user) = current_user.as_ref() {
            if !current_user.is_satgas() {
                return unauthorized();
            }
        }

        let district_name = parq.district_name.map(|a| util::title_case(a));

        let sresult = dao.search(
            district_name.as_ref().map(|a| a.as_str()),
            // parq.village_name,
            &parq.query,
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

    /// Add village_data.
    #[api_endpoint(
        path = "/village_data/add",
        auth = "required",
        mutable,
        accessor = "admin,user"
    )]
    pub fn add_village_data(query: AddVillageData) -> ApiResult<VillageData> {
        query.validate()?;

        let conn = state.db();
        let dao = VillageDataDao::new(&conn);

        let village = VillageDao::new(&conn).get_by_id(query.village_id)?;

        let mut last_updated_by_id = 0;
        let mut meta = vec![];

        if let Some(current_admin) = current_admin {
            if current_admin.id != 1 {
                if current_admin.get_city_id().unwrap_or(0) != village.city_id {
                    return param_error("Anda tidak memiliki akses untuk kab/kota ini");
                }
            }
            last_updated_by_id = current_admin.id;
            meta.push(format!("added_by_admin_id={}", current_admin.id));
        } else if let Some(current_user) = current_user {
            if !current_user.is_satgas() || !current_user.is_medic() {
                return unauthorized();
            }
            last_updated_by_id = current_user.id;
            meta.push(format!("added_by_user_id={}", current_user.id));
        }

        meta.push(format!("village={}", village.name));
        meta.push(format!("district={}", village.district_name));
        meta.push(format!("city={}", village.city));

        let village_data = dao.create(&NewVillageData {
            village_id: query.village_id,
            district_id: village.district_id,
            odp: query.record.odp,
            pdp: query.record.pdp,
            cases: query.record.cases,
            recovered: query.record.recovered,
            deaths: query.record.deaths,
            last_updated_by_id,
            city_id: village.city_id,
            meta: &meta.iter().map(|a| a.as_str()).collect(),
            ppdwt: query.record.ppdwt,
            pptb: query.record.pptb,
            odpsp: query.record.odpsp,
            pdps: query.record.pdps,
            pdpm: query.record.pdpm,
            otg: query.record.otg,
        })?;

        // meta.push(":updated_by_admin:".to_string());
        // meta.push(format!("updated_by_admin_name={}", current_admin.name));
        // meta.push(format!("updated_by_admin_id={}", current_admin.id));
        // meta.push(format!("village={}", village.name));
        // meta.push(format!("district={}", village.district_name));
        // meta.push(format!("city={}", village.city));

        // // update district data
        // {
        //     DistrictDataDao::new(&conn).update(
        //         village.district_id,
        //         Ops::Add,
        //         &UpdateDistrictData {
        //             odp: query.record.odp,
        //             pdp: query.record.pdp,
        //             cases: query.record.cases,
        //             recovered: query.record.recovered,
        //             deaths: query.record.deaths,

        //             ppdwt: query.record.ppdwt,
        //             pptb: query.record.pptb,
        //             odpsp: query.record.odpsp,
        //             pdps: query.record.pdps,
        //             pdpm: query.record.pdpm,
        //             otg: query.record.otg,

        //             city_id: village.city_id,
        //             last_updated_by_id: current_admin.id,
        //             meta: &meta.iter().map(|a| a.as_str()).collect(),
        //         },
        //     )?;
        // }

        Ok(ApiResult::success(village_data.to_api_type(&conn)))
    }

    /// Delete village data.
    #[api_endpoint(
        path = "/village_data/delete",
        auth = "required",
        mutable,
        accessor = "admin,user"
    )]
    pub fn delete_village_data(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = VillageDataDao::new(&conn);

        let d = dao.get_by_id(query.id)?;

        if let Some(current_admin) = current_admin {
            if current_admin.id != 1 && d.city_id != current_admin.get_city_id().unwrap_or(0) {
                return unauthorized();
            }
        } else if let Some(current_user) = current_user {
            if !current_user.is_satgas() || !current_user.is_medic() {
                return unauthorized();
            }
        }

        dao.delete_by_id(query.id)?;

        // // recalculate district data
        // DistrictDataDao::new(&conn).recalculate(
        //     d.city_id,
        //     d.district_id,
        //     d.village_id,
        //     current_admin.id,
        // )?;

        Ok(ApiResult::success(()))
    }

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
    #[api_endpoint(path = "/commit", auth = "required", mutable, accessor = "admin,user")]
    pub fn commit(query: CommitData) -> ApiResult<()> {
        use crate::schema::village_data::{self, dsl};
        query.validate()?;

        let conn = state.db();

        let locs = query
            .records
            .iter()
            .map(|a| format!("'{}/{}'", a.district_name, a.village_name))
            .collect::<Vec<String>>();

        let mut last_updated_by_id = 0;
        let mut last_updated_by_name = "";

        if let Some(current_admin) = current_admin.as_ref() {
            if current_admin.id != 1 {
                if !current_admin.has_access("update_village_data")
                    && current_admin.get_city_id().is_none()
                    && current_admin.id != 1
                {
                    return unauthorized();
                }

                last_updated_by_id = current_admin.id;
                last_updated_by_name = &current_admin.name;
            }
        }

        if let Some(current_user) = current_user.as_ref() {
            if !current_user.is_satgas() {
                return unauthorized();
            }
            if !current_user.is_medic() {
                return unauthorized();
            }
            if !current_user.has_access("village_data") {
                return unauthorized();
            }

            last_updated_by_id = current_user.id;
            last_updated_by_name = &current_user.full_name;
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
                    let mut meta: Vec<String> = {
                        village_data::table
                            .filter(dsl::id.eq(record.id))
                            .select(dsl::meta)
                            .first(&conn)?
                    };

                    meta = meta
                        .into_iter()
                        .filter(|a| !a.starts_with(":updated_by_"))
                        .collect();

                    // // if let Some(ca) = current_admin {
                    if current_admin.is_some() {
                        meta.push(":updated_by_admin:".to_string());
                    }
                    if current_user.is_some() {
                        meta.push(":updated_by_user:".to_string());
                        if current_user.as_ref().map(|a| a.is_medic()).unwrap_or(false) {
                            meta.push(":updated_by_medic:".to_string());
                        }
                    }
                    // meta.push(format!("updated_by_admin_name={}", current_admin.name));
                    // meta.push(format!("updated_by_admin_id={}", current_admin.id));
                    // // }

                    meta.dedup();

                    dao.update(
                        record.village_id,
                        Ops::Set,
                        &UpdateVillageData {
                            odp: record.odp,
                            pdp: record.pdp,
                            cases: record.cases,
                            recovered: record.recovered,
                            deaths: record.deaths,
                            last_updated_by_id,
                            meta: &meta.iter().map(|a| a.as_str()).collect(),
                            city_id: None,
                            district_id: None,
                            ppdwt: record.ppdwt,
                            pptb: record.pptb,
                            odpsp: record.odpsp,
                            pdps: record.pdps,
                            pdpm: record.pdpm,
                            otg: record.otg,
                        },
                    )?;

                    debug!("updating village data id {}...", record.id);

                    let village = VillageDao::new(&conn).get_by_id(record.village_id)?;

                    // // recalculate district data
                    // DistrictDataDao::new(&conn).recalculate(
                    //     village.city_id,
                    //     village.district_id,
                    //     village.id,
                    //     current_admin.id,
                    // )?;
                }

                Ok(())
            })?;

        Logs::new(&conn).write(
            &format!(
                "{} update village data for {} data: {}",
                last_updated_by_name,
                locs.len(),
                locs.join(", ")
            ),
            last_updated_by_id,
        );

        Ok(ApiResult::success(()))
    }

    /// Search for village_addresses
    #[api_endpoint(path = "/village_address", auth = "required")]
    pub fn search_village_address(query: VillageSearch) -> ApiResult<EntriesResult<VillageAddress>> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let sresult = dao.search(
            &query.query.unwrap_or("".to_string()),
            query.scope.as_ref().map(|a| a.as_str()),
            query.offset,
            query.limit,
        )?;

        let entries = sresult
            .entries
            .into_iter()
            .map(|a| VillageAddress {
                village_id: a.id,
                address: format!("{}, {}, {}, {}", a.name, a.city, a.district_name, a.province),
            })
            .collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }
}

#[derive(Serialize)]
pub struct VillageAddress {
    pub village_id: i64,
    pub address: String,
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
