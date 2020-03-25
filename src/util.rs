//! Utilitas yang berisi fungsi-fungsi kecil yang sering digunakan.
//! Contoh di sini kita bisa mendapatkan waktu terkini dalam milidetik dll.

use chrono::{NaiveDateTime, Utc};
use rand::{self, distributions::Alphanumeric, Rng};
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Mendapatkan waktu saat ini dalam format milidetik sejak UNIX EPOCH.
pub fn current_time_millis() -> u64 {
    current_time().as_secs() as u64 * 1000 as u64
}

/// Mendapatkan waktu saat ini dalam format milidetik sejak UNIX EPOCH.
pub fn current_time() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
}

/// Get current time in NaiveDateTime
pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

/// Menggenerasikan string secara acak
/// sepanjang `length`.
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect::<String>()
}

/// Menggenerasikan angka acak untuk tipe f64
pub fn random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 10)
}

/// Menggenerasikan angka acak untuk tipe f64
pub fn random_number_f64() -> f64 {
    let mut rng = rand::thread_rng();
    //     rng.gen_range(0, 100) as f64
    f64::from(rng.gen_range(0, 100))
}

/// Wait or blocking for n millis
#[inline]
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}
