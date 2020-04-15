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

/// Convert any case to Title Case
#[inline(always)]
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|w| w.chars())
        .map(|mut c| {
            c.next()
                .into_iter()
                .flat_map(|c| c.to_uppercase())
                .chain(c.flat_map(|c| c.to_lowercase()))
        })
        .map(|c| c.collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("kenjeran").as_str(), "Kenjeran");
        assert_eq!(title_case("jawa tengah").as_str(), "Jawa Tengah");
        assert_eq!(
            title_case("daerah istimeWa yogyakARTA").as_str(),
            "Daerah Istimewa Yogyakarta"
        );
    }
}
