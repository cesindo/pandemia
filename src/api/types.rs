//! API message types
//!
#![doc(hidden)]

use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

// use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api::{self, ApiResult},
    error::{Error, ErrorCode},
    models,
    prelude::*,
    types::SubReportStatus,
    ID,
};

pub trait ToApiType<T> {
    // Convert db model into api type
    // updated:
    // menambahkan parameter user untuk mengetahui status
    // apakah user sudah menyukai feed/comment
    fn to_api_type(&self, conn: &PgConnection) -> T;
    // fn to_api_type2(&self, params: i32, conn: &PgConnection) -> T {
    //     self.to_api_type(conn)
    // }
}

#[derive(Serialize, Deserialize, Default)]
pub struct EntriesResult<T> {
    pub entries: Vec<T>,
    pub count: i64,
}

#[derive(Deserialize, Validate)]
pub struct QueryEntries {
    pub query: Option<String>,
    #[validate(range(min = 0, max = 1_000_000))]
    pub offset: i64,
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IdQuery {
    pub id: ID,
}

#[derive(Deserialize, Validate)]
pub struct ResetPassword {
    #[validate(email(message = "Email not valid, please enter valid email address"))]
    pub email: String,
    pub code: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LocationQuery {
    #[deprecated(since = "0.2.10", note = "use loc_path instead")]
    #[validate(length(max = 100))]
    pub loc: Option<String>,

    #[validate(length(max = 500))]
    pub loc_path: Option<String>,
    pub with_history: Option<bool>,
}

#[derive(Serialize, Validate)]
pub struct LocationInfoResult {
    pub name: String,
    pub odp: i32,
    pub pdp: i32,
    pub positive: i32,
    pub death: i32,
    pub recovered: i32,
}

#[derive(Deserialize, Validate)]
pub struct UserConnect {
    #[validate(length(min = 1, message = "Device id can't be empty"))]
    pub device_id: String,
    #[validate(length(min = 1, message = "Client app id can't be empty"))]
    pub app_id: String,
    #[validate(length(min = 1, message = "Provider name must be set, eg: android, apple"))]
    pub provider_name: String,
    #[validate(length(min = 1, message = "Location name can't be empty"))]
    pub loc_name: String,
    pub loc_name_full: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateLocation {
    #[validate(length(min = 1, max = 1000, message = "Device id can't be empty"))]
    pub device_id: String,
    #[validate(length(min = 1, max = 1000, message = "Location name can't be empty"))]
    pub loc_name: String,
    #[validate(length(min = 0, max = 1000, message = "Location full name can't be empty"))]
    pub loc_name_full: String,
}

#[derive(Serialize)]
pub struct PandemicInfoDetail {
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
}

#[derive(Serialize)]
pub struct OccupationInfoDetail {
    pub vac_total: i32,
    pub used_total: i32,
    pub waiting: i32,
    pub last_updated: String,
}

#[derive(Serialize)]
pub struct MapMarker {
    pub longitude: f64,
    pub latitude: f64,
    pub kind: i16,
    pub caption: String,
    pub desc: String,
    pub pandemic_detail: Option<PandemicInfoDetail>,
    pub occupation_detail: Option<OccupationInfoDetail>,
}

#[derive(Serialize)]
pub struct Record {
    pub id: ID,
    pub loc: String,
    pub loc_path: String,

    pub loc_kind: i16,
    pub loc_scope: String,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub latest: bool,
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
}

impl ToApiType<Record> for models::Record {
    fn to_api_type(&self, conn: &PgConnection) -> Record {
        let loc_scope = meta_value_str!(self, "loc_scope");
        Record {
            id: self.id,
            loc: self.loc.to_owned(),
            loc_path: self.loc_path.to_owned(),
            loc_kind: self.loc_kind,
            loc_scope: loc_scope.to_owned(),
            total_cases: self.total_cases,
            total_deaths: self.total_deaths,
            total_recovered: self.total_recovered,
            active_cases: self.active_cases,
            critical_cases: self.critical_cases,
            latest: self.latest,
            meta: self.meta.clone(),
            last_updated: self.last_updated,
            ppdwt: self.ppdwt,
            pptb: self.pptb,
            odp: self.odp,
            odpsp: self.odpsp,
            pdp: self.pdp,
            pdps: self.pdps,
            pdpm: self.pdpm,
            otg: self.otg,
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Admin {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone_num: String,
    pub accesses: Vec<String>,
    pub active: bool,
    pub register_time: NaiveDateTime,
    pub meta: Vec<String>,

    /// City
    pub city: String,
    pub city_id: Option<ID>,

    /// User's province
    pub province: String,
}

impl ToApiType<Admin> for models::Admin {
    fn to_api_type(&self, conn: &PgConnection) -> Admin {
        let accesses = self
            .meta
            .iter()
            .filter(|a| a.starts_with("access."))
            .map(|a| (&a[7..]).to_string())
            .collect();
        Admin {
            id: self.id,
            name: self.name.to_owned(),
            email: self.email.to_owned(),
            phone_num: self.phone_num.to_owned(),
            accesses,
            active: self.active,
            register_time: self.register_time,
            meta: self.meta.clone(),

            city: meta_value_str!(self, "city", "=").to_owned(),
            city_id: self.get_city_id(),
            province: meta_value_str!(self, "province", "=").to_owned(),
        }
    }
}

/// Bentuk model akun di dalam database.
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// ID dari akun.
    pub id: i64,

    /// Nama lengkap akun.
    pub full_name: String,

    /// Alamat email kun.
    pub email: String,

    /// Nomor telpon akun.
    pub phone_num: String,

    /// Waktu kapan akun ini didaftarkan.
    pub register_time: NaiveDateTime,

    /// Satgas
    pub is_satgas: bool,

    /// Is user activeated.
    pub active: bool,

    /// user roles
    pub roles: Vec<String>,

    /// Additional metadata
    pub meta: Vec<String>,

    /// Location latitude, longitude
    pub loc: models::LatLong,
    /// Flag whether this user (satgas) blocked.
    pub is_blocked: bool,

    /// Current user's village data if satgas
    pub village: String,

    /// whether this user is deleted
    pub is_deleted: bool,

    /// Location path
    pub loc_path: String,
}

impl From<models::User> for User {
    fn from(a: models::User) -> Self {
        let mut roles = vec![];

        if a.is_satgas() {
            roles.push("satgas".to_owned());
        }
        User {
            id: a.id,
            full_name: a.full_name.to_owned(),
            email: a.email.to_owned(),
            phone_num: a.phone_num.to_owned(),
            register_time: a.register_time,
            active: a.active,
            is_satgas: a.is_satgas() && !a.is_blocked(),
            roles,
            meta: a.meta.clone(),
            village: a.get_village_name().to_owned(),
            loc: a.get_lat_long(),
            is_blocked: a.is_blocked(),
            is_deleted: a.is_deleted(),
            loc_path: meta_value_str!(a, "loc_path", "=").to_owned(),
        }
    }
}

impl From<models::User> for ApiResult<User> {
    fn from(a: models::User) -> Self {
        ApiResult::success(a.into())
    }
}

/// Bentuk model akun di dalam database.
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Satgas {
    /// ID dari akun.
    pub id: i64,

    /// Nama lengkap akun.
    pub full_name: String,

    /// Alamat email kun.
    pub email: String,

    /// Nomor telpon akun.
    pub phone_num: String,

    /// Waktu kapan akun ini didaftarkan.
    pub register_time: NaiveDateTime,

    /// Is user activeated.
    pub active: bool,

    /// user roles
    pub roles: Vec<String>,

    /// Village
    pub village: String,

    /// User's village id
    pub village_id: i64,

    /// City
    pub city: String,

    /// User's province
    pub province: String,

    /// Location latitude, longitude
    pub loc: models::LatLong,

    /// Flag whether this user (satgas) blocked.
    pub blocked: bool,

    /// Daftar akses untuk satgasya
    pub accesses: Vec<String>,

    /// Is medic
    pub medic: bool,
}

impl ToApiType<Satgas> for models::User {
    fn to_api_type(&self, conn: &PgConnection) -> Satgas {
        let mut roles = vec![];

        if self.is_satgas() {
            roles.push("satgas".to_owned());
        }

        let accesses = self
            .meta
            .iter()
            .filter(|a| a.starts_with("access."))
            .map(|a| (&a[7..]).to_string())
            .collect();

        Satgas {
            id: self.id,
            full_name: self.full_name.to_owned(),
            email: self.email.to_owned(),
            phone_num: self.phone_num.to_owned(),
            register_time: self.register_time,
            active: self.active,
            roles: roles,
            village: meta_value_str!(self, "village", "=").to_owned(),
            village_id: meta_value_i64!(self, "village_id").unwrap_or(0),
            city: meta_value_str!(self, "city_name", "=").to_owned(),
            province: meta_value_str!(self, "province_name", "=").to_owned(),
            loc: self.get_lat_long(),
            blocked: self.is_blocked(),
            accesses,
            medic: self.is_medic(),
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct ReportNote {
    pub id: ID,
    pub title: String,
    pub notes: String,
    pub creator_id: ID,
    pub creator_name: String,
    // pub area_code: String,
    // pub meta: Vec<String>,
    pub ts: NaiveDateTime,
    // ------
    pub location: String,
    pub status: Vec<String>,
}

impl ToApiType<ReportNote> for models::ReportNote {
    fn to_api_type(&self, conn: &PgConnection) -> ReportNote {
        let location = meta_value_str!(self, "location", "=").to_owned();
        let mut status = vec![];
        if self.published {
            status.push("published".to_string());
        } else {
            status.push("unpublished".to_string());
        }
        ReportNote {
            id: self.id,
            title: self.title.to_owned(),
            notes: self.notes.to_owned(),
            creator_id: self.creator_id,
            creator_name: self.creator_name.to_owned(),
            location,
            ts: self.ts,
            status,
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct VillageData {
    pub id: ID,
    pub village_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub ts: NaiveDateTime,
    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,

    //----
    pub village_name: String,
    pub district_name: String,
}

impl From<(models::VillageData, models::Village)> for VillageData {
    fn from(a: (models::VillageData, models::Village)) -> Self {
        VillageData {
            id: a.0.id,
            village_id: a.0.village_id,
            odp: a.0.odp,
            pdp: a.0.pdp,
            cases: a.0.cases,
            recovered: a.0.recovered,
            deaths: a.0.deaths,
            last_updated: a.0.last_updated,
            last_updated_by_id: a.0.last_updated_by_id,
            ts: a.0.ts,
            // area_code: a.0.area_code.to_owned(),
            village_name: a.1.name.to_owned(),
            district_name: a.1.district_name.to_owned(),
            ppdwt: a.0.ppdwt,
            pptb: a.0.pptb,
            odpsp: a.0.odpsp,
            pdps: a.0.pdps,
            pdpm: a.0.pdpm,
            otg: a.0.otg,
        }
    }
}

impl ToApiType<VillageData> for models::VillageData {
    fn to_api_type(&self, conn: &PgConnection) -> VillageData {
        use crate::schema::villages::{self, dsl};
        let (village_name, district_name) = {
            villages::table
                .filter(dsl::id.eq(self.village_id))
                .select((dsl::name, dsl::district_name))
                .first::<(String, String)>(conn)
                .unwrap_or_else(|_| ("".to_string(), "".to_string()))
        };
        VillageData {
            id: self.id,
            village_id: self.village_id,
            odp: self.odp,
            pdp: self.pdp,
            cases: self.cases,
            recovered: self.recovered,
            deaths: self.deaths,
            last_updated: self.last_updated,
            last_updated_by_id: self.last_updated_by_id,
            ts: self.ts,
            ppdwt: self.ppdwt,
            pptb: self.pptb,
            odpsp: self.odpsp,
            pdps: self.pdps,
            pdpm: self.pdpm,
            otg: self.otg,
            village_name,
            district_name,
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct DistrictData {
    pub id: ID,
    pub district_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub ts: NaiveDateTime,
    //----
    pub district_name: String,
}

impl From<(models::DistrictData, models::District)> for DistrictData {
    fn from(a: (models::DistrictData, models::District)) -> Self {
        DistrictData {
            id: a.0.id,
            district_id: a.0.district_id,
            odp: a.0.odp,
            pdp: a.0.pdp,
            cases: a.0.cases,
            recovered: a.0.recovered,
            deaths: a.0.deaths,
            last_updated: a.0.last_updated,
            last_updated_by_id: a.0.last_updated_by_id,
            ts: a.0.ts,
            district_name: a.1.name.to_owned(),
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct SubReport {
    pub id: ID,
    pub creator_id: ID,
    pub creator_name: String,
    pub full_name: String,
    pub age: i32,
    pub residence_address: String,
    pub gender: String,
    pub coming_from: String,
    pub arrival_date: NaiveDate,
    pub healty: i32,
    pub notes: String,
    pub status: String,
    // pub meta: Vec<String>,
    pub ts: NaiveDateTime,
    // pub city_id: ID,
    // ------
    pub healthy_notes: String,
    pub created_by_admin: bool,
    pub reporter_village: String,
    pub reporter_district: String,
    pub from_red_zone: bool,
    pub has_symptoms: bool,
    pub traveler: bool,
}

impl ToApiType<SubReport> for models::SubReport {
    fn to_api_type(&self, conn: &PgConnection) -> SubReport {
        let status: SubReportStatus = self.status.into();
        let healthy_notes = meta_value_str!(self, "gejala", "=").to_owned();
        SubReport {
            id: self.id,
            creator_id: self.creator_id,
            creator_name: self.creator_name.to_owned(),
            full_name: self.full_name.to_owned(),
            age: self.age,
            residence_address: self.residence_address.to_owned(),
            gender: self.gender.to_owned(),
            coming_from: self.coming_from.to_owned(),
            arrival_date: self.arrival_date,
            healty: self.healty,
            notes: self.notes.to_owned(),
            status: format!("{}", status),
            // meta: self.meta.clone(),
            ts: self.ts,
            // city_id: self.city_id,
            healthy_notes,
            created_by_admin: list_has_flag!(self.meta, "updated_by_admin"),
            reporter_village: meta_value_str!(self, "village", "=").to_owned(),
            reporter_district: meta_value_str!(self, "district", "=").to_owned(),
            from_red_zone: list_has_flag!(self.meta, "from_red_zone"),
            has_symptoms: list_has_flag!(self.meta, "has_symptoms"),
            traveler: list_has_flag!(self.meta, "traveler"),
        }
    }
}
