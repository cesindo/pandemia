//! Geolocator helper functions
//!
use chrono::NaiveDateTime;
use diesel::prelude::*;
use reqwest;
use serde_json;

use crate::{result::Result, schema::geoloc_cache, sqlutil::lower, ID};

/// Latitude longitude representation struct
#[derive(Deserialize, Copy, Clone)]
pub struct LatLong {
    /// The latitude
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    /// The longitude
    #[serde(rename = "Longitude")]
    pub longitude: f64,
}

#[doc(hidden)]
#[derive(Deserialize)]
struct MetaInfo {
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
}

// #[derive(Deserialize)]
// struct DisplayPosition {
//     #[serde(rename = "Latitude")]
//     pub latitude: f64,
//     #[serde(rename = "Longitude")]
//     pub longitude: f64,
// }

#[doc(hidden)]
#[derive(Deserialize)]
struct Location {
    #[serde(rename = "DisplayPosition")]
    pub display_position: LatLong,
}

#[doc(hidden)]
#[derive(Deserialize)]
struct MapResult {
    #[serde(rename = "MatchLevel")]
    pub match_level: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[doc(hidden)]
#[derive(Deserialize)]
struct View {
    #[serde(rename = "Result")]
    pub result: Vec<MapResult>,
}

#[doc(hidden)]
#[derive(Deserialize)]
struct GeocoderResponse {
    #[serde(rename = "MetaInfo")]
    pub meta_info: MetaInfo,
    #[serde(rename = "View")]
    pub view: Vec<View>,
}

#[doc(hidden)]
#[derive(Deserialize)]
struct GeocoderResponseWrapper {
    #[serde(rename = "Response")]
    pub response: GeocoderResponse,
}

/// this is db model for geolocation cache
#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct GeolocCache {
    pub id: ID,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub ts: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "geoloc_cache"]
struct NewGeolocCache<'a> {
    name: &'a str,
    latitude: f64,
    longitude: f64,
}

/// Get latitude longitude from query location name like city etc.
pub fn loc_to_ll(query: &str, conn: &PgConnection) -> Result<LatLong> {
    use crate::schema::geoloc_cache::{self, dsl};

    // coba ambil dulu dari cache apabila ada
    if let Ok(geoloc) = geoloc_cache::table
        .filter(lower(dsl::name).eq(query.to_lowercase()))
        .select((dsl::latitude, dsl::longitude))
        .first::<(f64, f64)>(conn)
    {
        return Ok(LatLong {
            latitude: geoloc.0,
            longitude: geoloc.1,
        });
    }

    // tidak ada di cache, ambil dari source luar
    let query = normalize_query(query.to_lowercase());

    let mut resp = reqwest::get(&format!(
        "https://geocoder.ls.hereapi.com/6.2/geocode.json?apiKey={}&searchtext={}",
        env!("GEOLOCATOR_API_KEY"),
        query
    ))?;
    let resp_text = resp.text()?;
    let item: GeocoderResponseWrapper = serde_json::from_str(&resp_text)?;
    let latlong = item.response.view[0].result[0].location.display_position;

    // simpan dalam cache
    {
        if let Err(e) = diesel::insert_into(geoloc_cache::table)
            .values(&NewGeolocCache {
                name: &query,
                latitude: latlong.latitude,
                longitude: latlong.longitude,
            })
            .execute(conn)
        {
            error!("Cannot insert new cache for location query {}", query);
        }
    }

    Ok(latlong)
}

fn normalize_query<T: AsRef<str>>(query: T) -> String {
    query
        .as_ref()
        .replace("kab.", "")
        .replace("kota", "")
        .replace("kabupaten", "")
        .replace("provinsi", "")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_query() {
        assert_eq!(normalize_query("kabupaten wonosobo"), "wonosobo");
        assert_eq!(normalize_query("provinsi kalimantan utara"), "kalimantan utara");
    }
}
