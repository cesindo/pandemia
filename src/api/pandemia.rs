//! Koleksi query yang digunakan untuk operasi pada rest API Pandemia
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::{
    sql_query,
    sql_types::{BigInt, Integer, SmallInt},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::HashMap;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::*, parsed_query::*, ApiResult, Error as ApiError, Error::*, HttpRequest as ApiHttpRequest},
    auth,
    dao::{
        CityDao, DistrictDao, DistrictDataDao, Logs, RecordDao, ReportNoteDao, SubReportDao, VillageDao,
        VillageDataDao,
    },
    district_data_dao::{NewDistrictData, UpdateDistrictData},
    error::{self, Error, ErrorCode},
    eventstream::{self, Event::NewRecordUpdate},
    models,
    prelude::*,
    record_dao::MutateRecord,
    sub_report_dao,
    types::{HealthyKind, LocKind, Ops, SubReportStatus},
    village_data_dao::{NewVillageData, UpdateVillageData},
    ID,
};

#[derive(Deserialize, Validate)]
pub struct RecordUpdate {
    #[validate(range(min = 1, max = 9999999999))]
    pub id: i64,
    #[validate(length(min = 2, max = 1000))]
    pub loc: String,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub meta: Vec<String>,
    pub last_updated: NaiveDateTime,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odp: i32,
    pub odpsp: i32,
    pub pdp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
    pub loc_path: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateRecords {
    records: Vec<RecordUpdate>,
}

#[derive(Serialize)]
pub struct InfoLocation {
    pub name: String,
    pub latest_record: models::Record,
    pub history: Vec<models::Record>,
}

#[derive(Serialize, Deserialize)]
pub struct SubReportQuery {
    pub offset: i64,
    pub limit: i64,
    pub query: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Validate)]
pub struct AddRecord {
    #[validate(length(min = 2, max = 1000))]
    pub loc: String,

    #[validate(range(min = 0))]
    pub loc_id: i64,

    #[validate(range(min = 0, max = 20))]
    pub loc_kind: i16,

    // #[validate(length(min = 2, max = 1000))]
    // pub loc_scope: String,
    #[validate(range(min = 0))]
    pub total_cases: i32,
    #[validate(range(min = 0))]
    pub total_deaths: i32,
    #[validate(range(min = 0))]
    pub total_recovered: i32,
    #[validate(range(min = 0))]
    pub active_cases: i32,
    #[validate(range(min = 0))]
    pub critical_cases: i32,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odp: i32,
    pub odpsp: i32,
    pub pdp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,

    pub loc_path: String,
}

#[derive(Deserialize, Validate)]
pub struct AddSubReport {
    #[validate(length(min = 1, max = 50, message = "Nama wajib diisi"))]
    pub full_name: String,
    pub age: i32,
    #[validate(length(min = 1, max = 80, message = "Alamat tempat tinggal wajib diisi"))]
    pub residence_address: String,
    #[validate(length(min = 1, max = 50))]
    pub gender: String,
    #[validate(length(max = 70))]
    pub coming_from: String,
    pub arrival_date: Option<NaiveDate>,
    #[validate(length(max = 500))]
    pub notes: String,
    pub status: String,
    pub complaint: Option<Vec<String>>,
    pub add_info: Option<Vec<String>>,
    #[validate(length(max = 100))]
    pub village_name: Option<String>,
    #[validate(length(max = 100))]
    pub district_name: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateSubReport {
    pub id: ID,
    #[validate(length(min = 1, max = 50))]
    pub full_name: String,
    pub age: i32,
    #[validate(length(min = 1, max = 80))]
    pub residence_address: String,
    #[validate(length(min = 1, max = 50))]
    pub gender: String,
    #[validate(length(min = 1, max = 70))]
    pub coming_from: String,
    pub arrival_date: Option<NaiveDate>,
    #[validate(length(min = 1, max = 50))]
    pub notes: String,
    pub status: String,
    pub complaint: Option<Vec<String>>,
    pub add_info: Option<Vec<String>>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateReportNoteStatus {
    pub id: ID,
    pub state: Vec<String>,
}

macro_rules! has_label {
    ($lst:expr, $lbl:literal) => {
        $lst.iter()
            .map(|a| a.as_str())
            .collect::<Vec<&str>>()
            .contains(&$lbl)
    };
}

#[derive(Serialize)]
pub struct SubReportCount {
    // #[sql_type = "BigInt"]
    pub odp: i64,
    // #[sql_type = "BigInt"]
    pub pdp: i64,
    // #[sql_type = "BigInt"]
    pub otg: i64,
}

#[derive(QueryableByName)]
pub struct CountByStatus {
    #[sql_type = "Integer"]
    pub status: i32,
    #[sql_type = "BigInt"]
    pub count: i64,
}

/// Holder untuk implementasi API endpoint publik untuk Pandemia.
pub struct PublicApi;

#[api_group("Pandemia", "public", base = "/pandemia/v1")]
impl PublicApi {
    /// Add record.
    #[api_endpoint(path = "/add_record", auth = "required", mutable, accessor = "admin")]
    pub fn add_record(query: AddRecord) -> ApiResult<models::Record> {
        query.validate()?;
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        if !current_admin.has_access("records") {
            return unauthorized();
        }

        // let loc_path = query.loc_path;

        let record = dao.create(
            &MutateRecord {
                loc: &query.loc,
                loc_kind: query.loc_kind.into(),
                total_cases: query.total_cases,
                total_deaths: query.total_deaths,
                total_recovered: query.total_recovered,
                active_cases: query.active_cases,
                critical_cases: query.critical_cases,
                meta: vec![],
                ppdwt: query.ppdwt,
                pptb: query.pptb,
                odp: query.odp,
                odpsp: query.odpsp,
                pdp: query.pdp,
                pdps: query.pdps,
                pdpm: query.pdpm,
                otg: query.otg,
                loc_path: &query.loc_path,
                latest: true,
            },
            false,
        )?;

        eventstream::emit(NewRecordUpdate(None, record.clone()));

        Logs::new(&conn).write(
            &format!(
                "{} add new record for {}. total_cases: {}, total_deaths: {}, total_recovered: {}",
                current_admin.name, query.loc, query.total_cases, query.total_deaths, query.total_recovered
            ),
            current_admin.id,
        );

        Ok(ApiResult::success(record))
    }

    /// Add Sub Report.
    #[api_endpoint(path = "/sub_report/add", auth = "required", accessor = "user,admin", mutable)]
    pub fn add_sub_report(query: AddSubReport) -> ApiResult<models::SubReport> {
        query.validate()?;
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        let mut city_id = 0;
        let mut district_id = 0;
        let mut village_id = 0;
        let mut current_user_id = 0;
        let mut reporter_full_name = "";
        let mut village_name = String::from("");
        let mut district_name = String::from("");

        if let Some(current_user) = current_user.as_ref() {
            if !current_user.is_satgas() {
                return param_error("Anda tidak dapat menambahkan data");
            }

            if current_user.is_blocked() || current_user.is_deleted() {
                return unauthorized();
            }

            // let area_code = match current_user.get_area_code() {
            //     "" => return param_error("Anda tidak terdaftar pada area manapun"),
            //     a => a,
            // };
            city_id = match current_user.get_city_id() {
                Some(a) => a,
                None => return param_error("Anda tidak terdaftar sebagai satgas"),
            };

            // village_id = current_user
            //     .get_village_id()
            //     .ok_or(ApiError::InvalidParameter(4840, "no village_id".to_string()))?;

            current_user_id = current_user.id;

            village_id = current_user
                .get_village_id()
                .ok_or(Error::InvalidParameter("Has no village id".to_string()))?;

            district_id = current_user
                .get_district_id()
                .ok_or(Error::InvalidParameter("Has no district id".to_string()))?;

            reporter_full_name = &current_user.full_name;

            village_name = current_user.get_village_name().to_owned();
            district_name = current_user.get_district_name().to_owned();
        }

        if let Some(current_admin) = current_admin.as_ref() {
            if current_admin.id != 1 {
                city_id = match current_admin.get_city_id() {
                    Some(a) => a,
                    None => return unauthorized(),
                };
            }

            if query.village_name.is_none() {
                return param_error("Nama desa belum diset");
            }

            let _village_name = query.village_name.as_ref().unwrap();

            if query.district_name.is_none() || query.district_name == Some("".to_string()) {
                return param_error("Nama kecamatan belum diset");
            }
            let _district_name = query.district_name.as_ref().unwrap();

            let district = DistrictDao::new(&conn)
                .get_by_name(city_id, _district_name)
                .map_err(|_| {
                    ApiError::NotFound(404, format!("Tidak ada kecamatan dengan nama {}", _district_name))
                })?;

            let village = VillageDao::new(&conn)
                .get_by_name_id(city_id, district.id, _village_name)
                .map_err(|e| {
                    ApiError::BadRequest(
                        48231,
                        format!("Desa {} tidak terdaftar di dalam kota Anda", village_name),
                    )
                })?;

            village_id = village.id;
            village_name = village.name.to_owned();
            district_id = district.id;

            reporter_full_name = &current_admin.name;
        }

        let mut healthy: HealthyKind = HealthyKind::Health;
        let mut meta: Vec<String> = Vec::new();
        if let Some(complaint) = &query.complaint.as_ref() {
            if complaint.len() > 0 {
                healthy = HealthyKind::Sick;
                meta.push(format!("gejala={}", complaint.join(",")))
            }
        }

        if let Some(ca) = current_admin.as_ref() {
            meta.push(":updated_by_admin:".to_string());
            meta.push(format!("updated_by_admin_name={}", ca.name));
            meta.push(format!("updated_by_admin_id={}", ca.id));
        }

        let mut ppdwt = 0;
        let mut pptb = 0;
        // let mut otg = 0;

        meta.push(format!("village={}", village_name));
        meta.push(format!("district={}", district_name));
        if let Some(add_info) = query.add_info.as_ref() {
            if has_label!(add_info, "traveler") {
                meta.push(format!(":traveler:"));
                ppdwt = 1;
            }
            if has_label!(add_info, "from_red_zone") {
                meta.push(format!(":from_red_zone:"));
                ppdwt = 1;
            }
            if has_label!(add_info, "has_symptoms") {
                meta.push(format!(":has_symptoms:"));
            } else {
                pptb = 1;
            }
        }

        let status: SubReportStatus = SubReportStatus::from(&query.status);

        if status == SubReportStatus::Positive
            || status == SubReportStatus::Recovered
            || status == SubReportStatus::Death
        {
            // status ini hanya boleh admin ke-atas
            if current_user.is_some() {
                return unauthorized();
            }
        }

        conn.build_transaction()
            .read_write()
            .run::<_, crate::error::Error, _>(|| {
                let sub_report = dao.create(
                    current_user_id,
                    reporter_full_name,
                    &query.full_name,
                    query.age,
                    &query.residence_address,
                    &query.gender,
                    &query.coming_from,
                    query.arrival_date,
                    healthy as i32,
                    &query.notes,
                    status as i32,
                    &meta.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
                    city_id,
                    district_id,
                    village_id,
                )?;

                {
                    // let mut meta = vec![];

                    // if let Some(ca) = current_admin.as_ref() {
                    //     meta.push(":updated_by_admin:".to_string());
                    //     meta.push(format!("updated_by_admin_name={}", ca.name));
                    //     meta.push(format!("updated_by_admin_id={}", ca.id));
                    // }

                    // meta.push(format!("village={}", village_name));
                    // meta.push(format!("district={}", district_name));

                    debug!(
                        "city_id={}, district_id={}, village_id={}",
                        city_id, district_id, village_id
                    );

                    let (odp, pdp, cases, recovered, deaths, otg) = match status {
                        SubReportStatus::ODP => (1, 0, 0, 0, 0, 0),
                        SubReportStatus::PDP => (0, 1, 0, 0, 0, 0),
                        SubReportStatus::Positive => (0, 0, 1, 0, 0, 0),
                        SubReportStatus::Recovered => (0, 0, 0, 1, 0, 0),
                        SubReportStatus::Death => (0, 0, 0, 0, 1, 0),
                        SubReportStatus::OTG => (0, 0, 0, 0, 0, 1),
                        _ => return Err(Error::InvalidParameter("Status tidak valid".to_owned()))?,
                    };

                    VillageDataDao::new(&conn).update(
                        village_id,
                        Ops::Add,
                        &UpdateVillageData {
                            odp,
                            pdp,
                            cases,
                            recovered,
                            deaths,
                            last_updated_by_id: current_user_id,
                            meta: &meta.iter().map(|a| a.as_str()).collect(),
                            city_id: Some(city_id),
                            district_id: Some(district_id),
                            ppdwt,
                            pptb,
                            odpsp: 0,
                            pdps: 0,
                            pdpm: 0,
                            otg,
                        },
                    )?;

                    DistrictDataDao::new(&conn).update(
                        district_id,
                        Ops::Add,
                        &UpdateDistrictData {
                            odp,
                            pdp,
                            cases,
                            recovered,
                            deaths,
                            last_updated_by_id: current_user_id,
                            city_id,
                            meta: &meta.iter().map(|a| a.as_str()).collect(),
                            ppdwt,
                            pptb,
                            odpsp: 0,
                            pdps: 0,
                            pdpm: 0,
                            otg,
                        },
                    )?;
                }

                Ok(ApiResult::success(sub_report))
            })
            .map_err(From::from)
    }

    /// Delete sub report.
    #[api_endpoint(
        path = "/sub_report/delete",
        auth = "required",
        mutable,
        accessor = "user,admin"
    )]
    pub fn delete_sub_report(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        let sr = dao.get_by_id(query.id)?;

        let mut current_user_id = 0;
        // let mut city_id = 0;
        // let mut village_id = 0;

        if let Some(current_user) = current_user {
            if !current_user.is_satgas() || current_user.is_blocked() || current_user.is_deleted() {
                return unauthorized();
            }
            let village_id = match current_user.get_village_id() {
                Some(a) => a,
                None => return unauthorized(),
            };
            if sr.village_id != village_id {
                return unauthorized();
            }
            current_user_id = current_user.id;
        // city_id = current_user.get_city_id().ok_or(ApiError::Unauthorized)?;
        } else if let Some(current_admin) = current_admin {
            if current_admin.id != 1 && !current_admin.has_access("update_village_data") {
                let city_id = match current_admin.get_city_id() {
                    Some(a) => a,
                    None => return unauthorized(),
                };
                if sr.city_id != city_id {
                    return unauthorized();
                }
            }
        } else {
            return unauthorized();
        }

        let mut ppdwt = 0;
        let mut pptb = 0;
        let odpsp = 0;
        // let mut otg = 0;
        let mut pdps = 0;
        let pdpm = 0;

        // meta.push(format!("village={}", village_name));
        // meta.push(format!("district={}", district_name));
        {
            if has_label!(sr.meta, "traveler") {
                ppdwt = -1;
            }
            if has_label!(sr.meta, "from_red_zone") {
                ppdwt = -1;
            }
            if has_label!(sr.meta, "has_symptoms") {
                pptb = -1;
                // otg = -1;
            }
        }

        if sr.status == SubReportStatus::Recovered as i32 {
            pdps = -1;
        }

        conn.build_transaction()
            .read_write()
            .run::<_, crate::error::Error, _>(|| {
                dao.delete_by_id(sr.id)?;

                let (odp, pdp, cases, recovered, deaths, otg) = match sr.status.into() {
                    SubReportStatus::ODP => (1, 0, 0, 0, 0, 0),
                    SubReportStatus::PDP => (0, 1, 0, 0, 0, 0),
                    SubReportStatus::Positive => (0, 0, 1, 0, 0, 0),
                    SubReportStatus::Recovered => (0, 0, 0, 1, 0, 0),
                    SubReportStatus::Death => (0, 0, 0, 0, 1, 0),
                    SubReportStatus::OTG => (0, 0, 0, 0, 0, 1),
                    _ => return Err(Error::InvalidParameter("Status tidak valid".to_owned()))?,
                };

                VillageDataDao::new(&conn).update(
                    sr.village_id,
                    Ops::Subs,
                    &UpdateVillageData {
                        odp,
                        pdp,
                        cases,
                        recovered,
                        deaths,
                        last_updated_by_id: current_user_id,
                        meta: &sr.meta.iter().map(|a| a.as_str()).collect(),
                        city_id: Some(sr.city_id),
                        district_id: Some(sr.district_id),
                        ppdwt,
                        pptb,
                        odpsp,
                        pdps,
                        pdpm,
                        otg,
                    },
                )?;

                DistrictDataDao::new(&conn).update(
                    sr.district_id,
                    Ops::Subs,
                    &UpdateDistrictData {
                        odp,
                        pdp,
                        cases,
                        recovered,
                        deaths,
                        last_updated_by_id: current_user_id,
                        city_id: sr.city_id,
                        meta: &sr.meta.iter().map(|a| a.as_str()).collect(),
                        ppdwt,
                        pptb,
                        odpsp,
                        pdps,
                        pdpm,
                        otg,
                    },
                )?;

                Ok(())
            })?;

        Ok(ApiResult::success(()))
    }

    /// Update Sub Report.
    #[api_endpoint(
        path = "/sub_report/update",
        auth = "required",
        accessor = "user,admin",
        mutable
    )]
    pub fn update_sub_report(query: UpdateSubReport) -> ApiResult<SubReport> {
        query.validate()?;
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        let mut current_user_id = 0;
        let mut city_id = 0;
        let mut village_id = 0;

        let subr = dao.get_by_id(query.id)?;

        let old_status: SubReportStatus = subr.status.into();

        if let Some(current_user) = current_user {
            if !current_user.is_satgas() {
                return param_error("Anda tidak dapat menambahkan data")?;
            }
            if current_user.is_blocked() || current_user.is_deleted() {
                return unauthorized();
            }
            // let area_code = match current_user.get_area_code() {
            //     "" => return param_error("Anda tidak terdaftar pada area manapun"),
            //     a => a,
            // };
            city_id = match current_user.get_city_id() {
                None => return param_error("Anda tidak terdaftar pada area manapun (no city_id)"),
                Some(a) => a,
            };
            village_id = match current_user.get_village_id() {
                None => return param_error("Anda tidak terdaftar pada area manapun (no village_id)"),
                Some(a) => a,
            };

            if subr.city_id != city_id {
                return unauthorized();
            }
            if subr.village_id != village_id {
                return unauthorized();
            }

            current_user_id = current_user.id;
        }

        // apabila admin hanya admin pada city tertentu yg boleh update
        if let Some(current_admin) = current_admin {
            if current_admin.id != 1 {
                city_id = match current_admin.get_city_id() {
                    None => return param_error("Anda tidak terdaftar pada area ini"),
                    Some(a) => a,
                };

                if subr.city_id != city_id {
                    return unauthorized();
                }
            }
        }

        let mut ppdwt = 0;
        let mut pptb = 0;
        // let mut otg = 0;
        let mut odpsp = 0;
        let mut pdps = 0;
        let mut pdpm = 0;

        let mut healthy: HealthyKind = HealthyKind::Health;

        let mut meta: Vec<String> = subr.meta.clone();

        if let Some(complaint) = &query.complaint.as_ref() {
            if complaint.len() > 0 {
                healthy = HealthyKind::Sick;
                meta.push(format!("gejala={}", complaint.join(",")))
            }
        }

        if let Some(add_info) = query.add_info.as_ref() {
            if has_label!(add_info, "traveler") {
                meta.push(format!(":traveler:"));
                ppdwt = 1;
            }
            if has_label!(add_info, "from_red_zone") {
                meta.push(format!(":from_red_zone:"));
                ppdwt = 1;
            }
            if has_label!(add_info, "has_symptoms") {
                meta.push(format!(":has_symptoms:"));
            } else {
                pptb = 1;
                // otg = 1;
            }
            if has_label!(add_info, "odpsp") {
                meta.push(format!(":odpsp:"));
                odpsp = 1;
            }
        }

        let new_status: SubReportStatus = query.status.as_str().into();

        if old_status == SubReportStatus::PDP && new_status == SubReportStatus::Recovered {
            pdps = 1;
        }
        if old_status == SubReportStatus::PDP && new_status == SubReportStatus::Death {
            pdpm = 1;
        }

        let sub_report = conn
            .build_transaction()
            .read_write()
            .run::<_, crate::error::Error, _>(|| {
                let village = VillageDao::new(&conn).get_by_id(village_id)?;

                meta.sort();
                meta.dedup();

                let sub_report = dao.update(
                    query.id,
                    sub_report_dao::UpdateSubReport {
                        full_name: &query.full_name,
                        age: query.age,
                        residence_address: &query.residence_address,
                        gender: &query.gender,
                        coming_from: &query.coming_from,
                        arrival_date: query.arrival_date,
                        healthy: healthy as i32,
                        notes: &query.notes,
                        status: new_status as i32,
                        meta: &meta.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
                        city_id,
                        district_id: subr.district_id,
                        village_id: village.id,
                    },
                )?;

                let (odp, pdp, cases, recovered, deaths, otg) = match old_status {
                    SubReportStatus::ODP => (1, 0, 0, 0, 0, 0),
                    SubReportStatus::PDP => (0, 1, 0, 0, 0, 0),
                    SubReportStatus::Positive => (0, 0, 1, 0, 0, 0),
                    SubReportStatus::Recovered => (0, 0, 0, 1, 0, 0),
                    SubReportStatus::Death => (0, 0, 0, 0, 1, 0),
                    SubReportStatus::OTG => (0, 0, 0, 0, 0, 1),
                    _ => return Err(Error::InvalidParameter("Status tidak valid".to_owned()))?,
                };

                VillageDataDao::new(&conn).update(
                    subr.village_id,
                    Ops::Subs,
                    &UpdateVillageData {
                        odp,
                        pdp,
                        cases,
                        recovered,
                        deaths,
                        last_updated_by_id: current_user_id,
                        meta: &subr.meta.iter().map(|a| a.as_str()).collect(),
                        city_id: Some(city_id),
                        district_id: Some(subr.district_id),
                        ppdwt,
                        pptb,
                        odpsp,
                        pdps,
                        pdpm,
                        otg,
                    },
                )?;

                DistrictDataDao::new(&conn).update(
                    subr.district_id,
                    Ops::Subs,
                    &UpdateDistrictData {
                        odp,
                        pdp,
                        cases,
                        recovered,
                        deaths,
                        last_updated_by_id: current_user_id,
                        city_id: subr.city_id,
                        meta: &subr.meta.iter().map(|a| a.as_str()).collect(),
                        ppdwt,
                        pptb,
                        odpsp,
                        pdps,
                        pdpm,
                        otg,
                    },
                )?;

                let (odp, pdp, cases, recovered, deaths, otg) = match new_status {
                    SubReportStatus::ODP => (1, 0, 0, 0, 0, 0),
                    SubReportStatus::PDP => (0, 1, 0, 0, 0, 0),
                    SubReportStatus::Positive => (0, 0, 1, 0, 0, 0),
                    SubReportStatus::Recovered => (0, 0, 0, 1, 0, 0),
                    SubReportStatus::Death => (0, 0, 0, 0, 1, 0),
                    SubReportStatus::OTG => (0, 0, 0, 0, 0, 1),
                    _ => return Err(Error::InvalidParameter("Status tidak valid".to_owned()))?,
                };

                VillageDataDao::new(&conn).update(
                    subr.village_id,
                    Ops::Add,
                    &UpdateVillageData {
                        odp,
                        pdp,
                        cases,
                        recovered,
                        deaths,
                        last_updated_by_id: current_user_id,
                        city_id: Some(subr.city_id),
                        district_id: Some(subr.district_id),
                        meta: &subr.meta.iter().map(|a| a.as_str()).collect(),
                        ppdwt,
                        pptb,
                        odpsp,
                        pdps,
                        pdpm,
                        otg,
                    },
                )?;

                DistrictDataDao::new(&conn).update(
                    subr.district_id,
                    Ops::Add,
                    &UpdateDistrictData {
                        odp,
                        pdp,
                        cases,
                        recovered,
                        deaths,
                        last_updated_by_id: current_user_id,
                        city_id: subr.city_id,
                        meta: &subr.meta.iter().map(|a| a.as_str()).collect(),
                        ppdwt,
                        pptb,
                        odpsp,
                        pdps,
                        pdpm,
                        otg,
                    },
                )?;
                Ok(sub_report)
            })?;

        Ok(ApiResult::success(sub_report.to_api_type(&conn)))
    }

    /// Search for sub_report
    #[api_endpoint(path = "/sub_report/search", auth = "required", accessor = "user,admin")]
    pub fn search_sub_reports(query: SubReportQuery) -> ApiResult<EntriesResult<SubReport>> {
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        // let area_code = match current_user.get_area_code() {
        //     "" => return param_error("Anda tidak terdaftar pada area manapun"),
        //     a => a,
        // };

        let mut parq = match query.query.as_ref() {
            Some(q) => parse_query(q),
            None => ParsedQuery::default(),
        };

        let current_user_name = current_user.as_ref().map(|a| a.full_name.to_owned()).unwrap_or(
            current_admin
                .as_ref()
                .map(|a| a.name.to_owned())
                .unwrap_or("".to_string()),
        );

        let (city_id, district_id, village_id) = if let Some(current_user) = current_user.as_ref() {
            match (
                current_user.get_city_id(),
                current_user.get_district_id(),
                current_user.get_village_id(),
            ) {
                (Some(a), Some(b), Some(c)) => (Some(a), Some(b), Some(c)),
                _ => return param_error("Anda tidak terdaftar pada area manapun (no city_id)"),
            }
        } else if current_admin.as_ref().map(|a| a.id == 1).unwrap_or(false) {
            // root admin tak perlu dibatasi city
            (None, None, None)
        } else if let Some(current_admin) = current_admin.as_ref() {
            match current_admin.get_city_id() {
                None => return param_error("Anda tidak terdaftar pada area manapun (no city_id)"),
                Some(city_id) => {
                    let city = CityDao::new(&conn).get_by_id(city_id)?;
                    let district = match parq
                        .district_name
                        .and_then(|name| DistrictDao::new(&conn).get_by_name(city_id, name).ok())
                    {
                        Some(district) => {
                            if district.city_id != city_id {
                                return bad_request(&format!(
                                    "Anda tidak memiliki akses untuk kecamatan {}",
                                    district.name
                                ));
                            }
                            Some(district)
                        }
                        None => None,
                    };

                    let village_id = parq
                        .village_name
                        .and_then(|name| {
                            VillageDao::new(&conn)
                                .get_by_name_id(city.id, district.as_ref().map(|a| a.id).unwrap_or(0), name)
                                .ok()
                        })
                        .map(|a| a.id);

                    (Some(city_id), district.map(|a| a.id), village_id)
                }
            }
        } else {
            return unauthorized();
        };

        let name = parq.name.unwrap_or("");

        // utamakan status dari param `status`
        let status: SubReportStatus = query.status.into();
        // dbg!(status);
        if status != SubReportStatus::Unknown {
            parq.status = Some(status);
        }

        let mut village_name = parq.village_name.map(|a| util::title_case(a));

        if let Some(village_id) = village_id {
            let village = VillageDao::new(&conn).get_by_id(village_id)?;
            if village_name.is_some() && village_name.as_ref().map(|a| a.as_str()) != Some("") {
                if Some(&village.name) != village_name.as_ref() {
                    debug!(
                        "user {} dengan akses desa {} mencoba mendapatkan data sub report untuk desa {}",
                        current_user_name,
                        village.name,
                        village_name.unwrap_or("??".to_string())
                    );
                    return Ok(ApiResult::success(EntriesResult {
                        count: 0,
                        entries: vec![],
                    }));
                }
            } else {
                village_name = Some(village.name.to_owned());
            }
        }

        let result = dao.search(
            city_id,
            district_id,
            village_name.as_ref().map(|a| a.as_str()),
            parq.come_from,
            parq.age,
            parq.residence_address,
            parq.gender,
            parq.status,
            &name,
            None,
            query.offset,
            query.limit,
        )?;

        Ok(ApiResult::success(EntriesResult {
            entries: result.entries.into_iter().map(|a| a.to_api_type(&conn)).collect(),
            count: result.count,
        }))
    }

    /// Get sub report all status count for current user village access.
    #[api_endpoint(path = "/sub_report/count", auth = "required")]
    pub fn get_sub_report_count(query: ()) -> ApiResult<JsonMap<String, JsonValue>> {
        let conn = state.db();

        let village_id = current_user
            .get_village_id()
            .ok_or(param_err!("You have no accesss to this resource"))?;

        let result: Vec<CountByStatus> = sql_query(&format!(
            "select status, COUNT(*) from sub_reports WHERE village_id={} GROUP BY status",
            village_id
        ))
        .load(&conn)
        .map_err(Error::from)?;

        let mut d: JsonMap<String, JsonValue> = JsonMap::new();

        for rs in result {
            let st: SubReportStatus = rs.status.into();
            d.insert(format!("{}", st), json!(rs.count as i32));
        }

        Ok(ApiResult::success(d))
    }

    /// Get location stats data (single mode).
    #[api_endpoint(path = "/info_location", auth = "none")]
    pub fn get_info_location(query: LocationQuery) -> ApiResult<Option<models::Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        // let locs: Vec<String> = vec![query.loc.to_owned()];

        if let Some(loc_path) = query.loc_path {
            let rec = dao.get_latest_record_one(&loc_path).ok();

            if let Some(rec) = rec {
                return Ok(ApiResult::success(Some(rec)));
            }
        }
        return Ok(ApiResult::success(None));
    }

    /// Get per location stats data, use comma for multiple locations.
    #[api_endpoint(path = "/info_locations", auth = "none")]
    pub fn get_info_locations(query: LocationQuery) -> ApiResult<Vec<InfoLocation>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let locs: Vec<String> = {
            if let Some(loc_path) = query.loc_path.as_ref() {
                loc_path
                    .split(',')
                    .into_iter()
                    .filter(|a| !a.is_empty())
                    .map(|a| a.to_string())
                    .collect()
            } else if let Some(loc) = query.loc.as_ref() {
                // for backward compatibility only
                // @TODO(Robin): remove this
                loc.split(',').map(|a| format!("/Indonesia/{}", a)).collect()
            } else {
                return param_error("No loc nor loc_path parameter provided");
            }
        };

        let records = dao.get_latest_records(&locs, 0, 10)?;

        let mut result = vec![];

        for rec in records {
            let mut history: Vec<models::Record> = vec![];

            if query.with_history == Some(true) {
                history = dao.get_record_history(&rec.loc, 0, 30)?;
            }

            result.push(InfoLocation {
                name: rec.loc.to_owned(),
                latest_record: rec,
                history,
            });
        }

        Ok(ApiResult::success(result))
    }

    /// Get latest data record search/query by location.
    #[api_endpoint(path = "/search_records", auth = "required", accessor = "admin")]
    pub fn search_records(query: QueryEntries) -> ApiResult<EntriesResult<Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let result = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult {
            count: result.count,
            entries: result.entries.into_iter().map(|a| a.to_api_type(&conn)).collect(),
        }))
    }

    /// Update multiple records at once.
    #[api_endpoint(path = "/update_records", auth = "required", mutable, accessor = "admin")]
    pub fn update_records(query: UpdateRecords) -> ApiResult<()> {
        use crate::schema::records::{self, dsl};
        query.validate()?;

        let conn = state.db();

        let locs = query
            .records
            .iter()
            .map(|a| a.loc.to_owned())
            .collect::<Vec<String>>();

        conn.build_transaction()
            .read_write()
            .run::<_, error::Error, _>(|| {
                let dao = RecordDao::new(&conn);

                for record in query.records {
                    let old_record = dao.get_latest_record_one(&record.loc_path).ok();

                    let new_record = dao.create(
                        &MutateRecord {
                            loc: &record.loc,
                            loc_kind: record.loc_kind.into(),
                            total_cases: record.total_cases,
                            total_deaths: record.total_deaths,
                            total_recovered: record.total_recovered,
                            active_cases: record.active_cases,
                            critical_cases: record.critical_cases,
                            meta: record.meta.iter().map(|a| a.as_str()).collect(),

                            ppdwt: record.ppdwt,
                            pptb: record.pptb,
                            odp: record.odp,
                            odpsp: record.odpsp,
                            pdp: record.pdp,
                            pdps: record.pdps,
                            pdpm: record.pdpm,
                            otg: record.otg,

                            latest: true,
                            loc_path: old_record.as_ref().map(|a| a.loc_path.as_str()).unwrap_or(""),
                        },
                        true,
                    )?;

                    if let Some(old_record) = old_record {
                        let diff = new_record.diff(&old_record);

                        if diff.new_cases > 0
                            || diff.new_deaths > 0
                            || diff.new_recovered > 0
                            || diff.new_critical > 0
                        {
                            eventstream::emit(NewRecordUpdate(Some(old_record.clone()), new_record.clone()));
                        }
                    }

                    debug!("updating record {}...", record.id);
                }

                Ok(())
            })?;

        Logs::new(&conn).write(
            &format!(
                "{} update record for {} records: {}",
                current_admin.name,
                locs.len(),
                locs.join(", ")
            ),
            current_admin.id,
        );

        Ok(ApiResult::success(()))
    }

    /// Delete record by id
    #[api_endpoint(path = "/delete_record", auth = "required", mutable, accessor = "admin")]
    pub fn delete_record(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        let rec = dao.get_by_id(query.id)?;
        dao.delete_by_id(rec.id)?;

        Logs::new(&conn).write(
            &format!("{} delete record for {}", current_admin.name, rec.loc),
            current_admin.id,
        );

        Ok(ApiResult::success(()))
    }

    /// Search for journal_logs
    #[api_endpoint(path = "/journal/search", auth = "required", accessor = "admin")]
    pub fn search_journal_logs(query: QueryEntries) -> ApiResult<EntriesResult<models::Log>> {
        let conn = state.db();
        let dao = Logs::new(&conn);

        let rv = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult {
            count: rv.count,
            entries: rv.entries,
        }))
    }

    /// Add report note.
    #[api_endpoint(path = "/report_note/add", auth = "required", mutable)]
    pub fn add_report_note(query: AddReportNote) -> ApiResult<ReportNote> {
        query.validate()?;
        let conn = state.db();
        let dao = ReportNoteDao::new(&conn);

        if current_user.is_blocked() {
            return unauthorized();
        }

        // let area_code = current_user.get_area_code();
        let city_id = current_user.get_city_id().ok_or(InvalidParameter(
            ErrorCode::Unauthorized as i32,
            "Tidak terdaftar sebagai satgas".to_string(),
        ))?;

        // let village = match VillageDao::new(&conn).get_by_name(&query.village) {
        //     Ok(a) => a,
        //     Err(_) => {
        //         return param_error(&format!(
        //             "Tidak dapat menemukan data untuk desa {}",
        //             query.village
        //         ))
        //     }
        // };

        let mut meta = vec![];

        // meta.push(format!("area_code={}", area_code));
        // meta.push(":reviewed:".to_string());
        meta.push(format!("location={}", current_user.get_village_name()));

        let report_note = dao.create(
            &query.title.unwrap_or("".to_string()),
            &query.notes,
            current_user.id,
            &current_user.full_name,
            city_id,
            &meta.iter().map(|a| a.as_str()).collect(),
        )?;
        Ok(ApiResult::success(report_note.to_api_type(&conn)))
    }

    /// Delete report note.
    #[api_endpoint(path = "/report_note/delete", auth = "required", mutable, accessor = "admin")]
    pub fn delete_report_note(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = ReportNoteDao::new(&conn);

        if !current_admin.has_access("report_notes") {
            return unauthorized();
        }

        let rnote = dao.get_by_id(query.id)?;

        if current_admin.id != 1 {
            if Some(rnote.city_id) != current_admin.get_city_id() {
                return unauthorized();
            }
        }

        dao.delete_by_id(rnote.id)?;

        Ok(ApiResult::success(()))
    }

    /// Update report note's approval status.
    #[api_endpoint(
        path = "/report_note/update_state",
        auth = "required",
        mutable,
        accessor = "admin"
    )]
    pub fn update_state_report_note(query: UpdateReportNoteStatus) -> ApiResult<()> {
        use crate::schema::report_notes::{self, dsl};

        let conn = state.db();
        let dao = ReportNoteDao::new(&conn);

        if !current_admin.has_access("report_notes") {
            return unauthorized();
        }

        let rnote = dao.get_by_id(query.id)?;

        if current_admin.id != 1 {
            if Some(rnote.city_id) != current_admin.get_city_id() {
                return unauthorized();
            }
        }

        let mut published = false;

        if query.state.contains(&"published".to_string()) {
            published = true;
        }
        diesel::update(dsl::report_notes.filter(dsl::id.eq(query.id)))
            .set(dsl::published.eq(published))
            .execute(&conn)
            .map_err(Error::from)?;

        Ok(ApiResult::success(()))
    }

    /// Search for report_notes
    #[api_endpoint(path = "/report_note/search", auth = "required", accessor = "admin")]
    pub fn search_report_notes(query: SearchNotes) -> ApiResult<EntriesResult<ReportNote>> {
        query.validate()?;
        let conn = state.db();
        let dao = ReportNoteDao::new(&conn);

        let city_id = current_admin.get_city_id();

        if city_id.is_none() && !current_admin.has_access("report_notes") {
            return unauthorized();
        }

        let city_id = city_id.unwrap_or(0);

        let sresult = dao.search(
            city_id,
            &query.query.unwrap_or("".to_string()),
            &query.state,
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
}

#[derive(Deserialize, Validate)]
pub struct SearchNotes {
    pub query: Option<String>,
    // pub meta_contains: String,
    pub state: String,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
}

#[derive(Deserialize, Validate)]
pub struct AddReportNote {
    pub title: Option<String>,
    pub notes: String,
}

use crate::{
    event_handler::FCM,
    push_notif_handler::{FCMHandler, FCMPayloadData},
    types::NotifKind,
    util,
};

use std::thread;

#[derive(Deserialize, Validate)]
pub struct TestPushNotifQuery {
    pub loc: String,
    pub loc_kind: i16,
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Pandemia", "private", base = "/pandemia/v1")]
impl PrivateApi {
    /// Test push notif functionality, only for internal testing purposes.
    #[api_endpoint(path = "/test/push_notif", auth = "none", mutable)]
    pub fn test_push_notif(query: TestPushNotifQuery) -> ApiResult<()> {
        let conn = state.db();
        let _ = thread::spawn(move || {
            if let Err(e) = FCM.push(
                "fcm",
                &FCMPayloadData {
                    receiver_loc: &query.loc,
                    receiver_loc_kind: query.loc_kind.into(),
                    target_id: 0,
                    kind: NotifKind::NewCases,
                    title: "Test",
                    item: "",
                    message: "This is test message",
                    created: util::now(),
                    click_action: "FLUTTER_NOTIFICATION_CLICK",
                },
                &conn,
            ) {
                error!("Cannot test send push notif. {}", e);
            }
        });

        Ok(ApiResult::success(()))
    }
}
