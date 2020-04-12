//! API message types
//!
#![doc(hidden)]

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

// use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api,
    error::{Error, ErrorCode},
    models,
    prelude::*,
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

#[derive(Serialize, Deserialize)]
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
    #[validate(length(min = 1, max = 100))]
    pub loc: String,
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
}

impl ToApiType<Record> for models::Record {
    fn to_api_type(&self, conn: &PgConnection) -> Record {
        let loc_scope = meta_value_str!(self, "loc_scope");
        Record {
            id: self.id,
            loc: self.loc.to_owned(),
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
        }
    }
}
