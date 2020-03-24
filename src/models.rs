//! Definisi struct untuk model-model yang ada di dalam database.

use crate::types::RecordDiff;
use chrono::NaiveDateTime;
use serde::Serialize;

use std::fmt;

use crate::ID;

/// Interface untuk model yang pasti memiliki id
/// sehingga bisa dijadikan model generik untuk mendapatkan id
pub trait HasID {
    /// Get this record ID.
    fn get_id(&self) -> ID;
}

/// Bentuk model akun di dalam database.
#[derive(Queryable, Clone, Serialize, PartialEq)]
pub struct User {
    /// ID dari akun.
    pub id: i64,

    /// Nama lengkap akun.
    pub full_name: String,

    /// Alamat email dari akun.
    pub email: String,

    /// Nomor telepon.
    pub phone_num: String,

    /// Penanda apakah akun aktif atau tidak,
    /// apabila tidak aktif maka akun tidak diperkenankan untuk beroperasi.
    pub active: bool,

    /// Waktu kapan akun ini didaftarkan.
    pub register_time: NaiveDateTime,
}

/// Bentuk model dari alamat untuk akun.
#[derive(Queryable)]
pub struct Address {
    /// ID dari record ini.
    pub id: i64,

    /// ID dari akun yang memiliki alamat ini.
    pub user_id: i64,

    /// Jenis alamat, 0: Domisili, 1: Kelahiran
    pub kind: i64,

    /// Alamat
    pub address: String,

    /// Kabupaten
    pub regency: String,

    /// Provinsi
    pub province: String,

    /// Negara
    pub country: String,

    /// Nomor telepon yang bisa dihubungi.
    pub phone_num: String,

    /// Penanda apakah alamat ini masih aktif atau tidak.
    pub active: bool,

    /// Catatan tentang alamat ini.
    pub notes: String,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct RegisterUser {
    // pub id: i64,
    pub token: String,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub register_time: NaiveDateTime,
    pub code: String,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, PartialEq, Debug)]
pub struct AccessToken {
    pub token: String,
    pub user_id: i64,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct UserPashash {
    pub user_id: i64,
    pub passhash: String,
    pub deperecated: bool,
    pub ver: i32,
    pub created: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct UserKey {
    pub id: ID,
    pub user_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User({}, {})", self.id, self.full_name)
    }
}

impl fmt::Display for UserKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({})", &self.pub_key[..8])
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Admin {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone_num: String,
    pub labels: Vec<String>,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct AdminAccessToken {
    pub token: String,
    pub admin_id: ID,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct ResetPasswordAdmin {
    pub admin_id: ID,
    pub token: String,
    pub created: NaiveDateTime,
    pub expiration: Option<NaiveDateTime>,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Record {
    pub id: ID,
    pub loc: String,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub cases_to_pop: f64,
    pub meta: Vec<String>,
    pub last_updated: NaiveDateTime,
}

impl Record {
    /// Get diff for this with other
    pub fn diff(&self, other: &Self) -> RecordDiff {
        let new_cases = self.total_cases - other.total_cases;
        let new_deaths = self.total_deaths - other.total_deaths;
        let new_recovered = self.total_recovered - other.total_recovered;
        let new_critical = self.critical_cases - other.critical_cases;

        RecordDiff {
            new_cases,
            new_deaths,
            new_recovered,
            new_critical,
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Notif {
    pub id: ID,
    pub kind: i16,
    pub text: String,
    pub initiator_id: ID,
    pub receiver_id: ID,
    pub read: bool,
    pub keywords: Vec<String>,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Feed {
    pub id: ID,
    pub creator_id: ID,
    pub creator_name: String,
    pub loc: String,
    pub kind: i16,
    pub text: String,
    pub hashtags: Vec<String>,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

/// Untuk serialize json dari server
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultItem {
    /// Field ID
    #[serde(rename = "FID")]
    pub fid: i64,

    /// Provinsi
    #[serde(rename = "Provinsi")]
    pub province: String,

    /// Jumlah Kasus Meninggal
    #[serde(rename = "Kasus_Meni")]
    pub total_deaths: i32,

    /// Jumlah Kasus Positif
    #[serde(rename = "Kasus_Posi")]
    pub active_cases: i32,

    /// Jumlah Kasus Sembuh
    #[serde(rename = "Kasus_Semb")]
    pub total_recovered: i32,
}

/// Untuk serialize json object dari server
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultObject {
    /// Field attributes
    pub attributes: ResultItem,
}