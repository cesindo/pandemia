//! Koleksi query yang digunakan untuk operasi pada rest API MapArea
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{expression::dsl::sql, sql_query, sql_types};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{error::param_error, ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    dao::MapMarkerDao,
    error::{Error, ErrorCode},
    models,
    prelude::*,
    types::MapMarkerKind,
    ID,
};

#[derive(Deserialize, Validate)]
pub struct SearchArea {
    pub longitude: f64,
    pub latitude: f64,
    pub query: Option<String>,
}

/// Holder untuk implementasi API endpoint publik untuk MapArea.
pub struct PublicApi;

#[api_group("MapArea", "public", base = "/map_area/v1")]
impl PublicApi {
    /// Mencari data pada radius 5km pada suatu wilayah menggunakan titik longlat.
    #[api_endpoint(path = "/search", auth = "none")]
    pub fn search_map_markers(query: SearchArea) -> ApiResult<Vec<MapMarker>> {
        use crate::schema::user_settings::{self, dsl};

        query.validate()?;

        let conn = state.db();

        let sql_text = format!(
            r#"u.id, u.full_name, s.s_key, s.s_value, uc.latest_loc_long, uc.latest_loc_lat FROM users as u INNER JOIN user_connect AS uc ON u.id=uc.user_id 
            INNER JOIN user_settings AS s ON s.user_id=uc.user_id
            WHERE s.s_key='complaint_map' AND s.s_value='true' AND 
            earth_box( ll_to_earth({}, {}), 10000/1.609 ) @> ll_to_earth(uc.latest_loc_lat, uc.latest_loc_long);"#,
            query.latitude, query.longitude
        );

        let u_data: Vec<(i64, String, String, String, f64, f64)> = diesel::select(sql::<(
            sql_types::Bigint,
            sql_types::VarChar,
            sql_types::Varchar,
            sql_types::VarChar,
            sql_types::Double,
            sql_types::Double,
        )>(&sql_text))
        .load(&conn)
        .map_err(Error::from)?;

        let mut map_markers = vec![];

        for (user_id, _, _, _, loc_long, loc_lat) in u_data {
            let user_settings: Vec<models::UserSetting> = user_settings::table
                .filter(dsl::user_id.eq(user_id))
                .load(&conn)
                .map_err(Error::from)?;

            let mut complaints = vec![];

            for copl in user_settings {
                if copl.s_key == "has_cough" && copl.s_value == "true" {
                    complaints.push("batuk");
                } else if copl.s_key == "has_fever" && copl.s_value == "true" {
                    complaints.push("demam");
                } else if copl.s_key == "has_cold" && copl.s_value == "true" {
                    complaints.push("flu");
                } else if copl.s_key == "has_headache" && copl.s_value == "true" {
                    complaints.push("pusing");
                }
            }

            map_markers.push(MapMarker {
                longitude: loc_long,
                latitude: loc_lat,
                kind: MapMarkerKind::Sick.into(),
                caption: "Keluhan".to_string(),
                desc: complaints.join(", "),
                pandemic_detail: None,
                occupation_detail: None,
            });
        }

        // get from map-markers
        {
            use crate::schema::map_markers::{self, dsl};
            // cari data daerah terdekat kecuali fasilitas kesehatan
            let pandemic_data: Result<Vec<models::MapMarker>> = map_markers::table
                .filter(sql(&format!(
                    "earth_box(ll_to_earth({}, {}), 10000/1.609) @> ll_to_earth(latitude, longitude) OR (kind=3 AND earth_box(ll_to_earth({}, {}), 100000/1.609) @> ll_to_earth(latitude, longitude))",
                    query.latitude, query.longitude,
                    query.latitude, query.longitude
                )))
                .load(&conn)
                .map_err(Error::from);

            match pandemic_data {
                Ok(mms) => {
                    for mm in mms {
                        let kind: MapMarkerKind = mm.kind.into();
                        let mut pandemic_detail = None;
                        let mut occupation_detail = None;
                        match kind {
                            MapMarkerKind::PandemicInfo => {
                                let total_cases: i32 = mm.get_meta_value_i32("pandemic.total_cases");
                                let total_deaths: i32 = mm.get_meta_value_i32("pandemic.total_deaths");
                                let total_recovered: i32 = mm.get_meta_value_i32("pandemic.total_recovered");

                                pandemic_detail = Some(PandemicInfoDetail {
                                    total_cases,
                                    total_deaths,
                                    total_recovered,
                                });
                            }
                            MapMarkerKind::Hospital => {
                                let used_total = mm.get_meta_value_i32("cekdiri.used_ttl");
                                let vac_total = mm.get_meta_value_i32("cekdiri.vac_ttl");
                                let waiting = mm.get_meta_value_i32("cekdiri.waiting");
                                let last_updated = mm.get_meta_value_str("cekdiri.last_updated");
                                occupation_detail = Some(OccupationInfoDetail {
                                    used_total,
                                    vac_total,
                                    waiting,
                                    last_updated: last_updated.to_owned(),
                                });
                            }
                            MapMarkerKind::Sick => {}
                            MapMarkerKind::Unknown => {}
                        }

                        map_markers.push(MapMarker {
                            longitude: mm.longitude,
                            latitude: mm.latitude,
                            kind: mm.kind.into(),
                            caption: mm.name.to_owned(),
                            desc: mm.info.to_owned(),
                            pandemic_detail,
                            occupation_detail,
                        });
                    }
                }
                Err(e) => error!("Cannot get map markers. {}", e),
            }
        }

        Ok(ApiResult::success(map_markers))
    }

    // /// Rest API endpoint untuk menambahkan map_area baru.
    // #[api_endpoint(path = "/add", mutable, auth = "required")]
    // pub fn add_map_area(query: NewMapArea) -> ApiResult<models::MapArea> {
    //     let conn = state.db();
    //     let dao = MapAreaDao::new(&conn);

    //     // @TODO(*): Add parameter checking here

    //     dao
    //         .create(
    //             &query.name,
    //         )
    //         .map_err(From::from)
    //         .map(ApiResult::success)
    // }

    // /// Mendapatkan daftar map_area
    // #[api_endpoint(path = "/list", auth = "required")]
    // pub fn list_map_area(query: QueryEntries) -> ApiResult<EntriesResult<models::MapArea>> {
    //     let conn = state.db();
    //     let dao = MapAreaDao::new(&conn);

    //     let entries = dao.get_map_areas(query.offset, query.limit)?;

    //     let count = dao.count()?;
    //     Ok(ApiResult::success(EntriesResult { count, entries }))
    // }

    // /// Mendapatkan jumlah map_area secara keseluruhan.
    // #[api_endpoint(path = "/count", auth = "required")]
    // pub fn map_area_count(state: &AppState, query: ()) -> ApiResult<i64> {
    //     let conn = state.db();
    //     let dao = MapAreaDao::new(&conn);

    //     dao.count().map(ApiResult::success).map_err(From::from)
    // }

    // /// Mendapatkan data map_area berdasarkan ID.
    // #[api_endpoint(path = "/detail", auth = "required")]
    // pub fn map_area_detail(query: IdQuery) -> ApiResult<models::MapArea> {
    //     let conn = state.db();
    //     let dao = MapAreaDao::new(&conn);

    //     dao.get_by_id(query.id)
    //         .map(ApiResult::success)
    //         .map_err(From::from)
    // }

    // /// Delete map_area.
    // #[api_endpoint(path = "/delete", auth = "required", mutable="true")]
    // pub fn delete_map_area(query: IdQuery) -> ApiResult<()> {
    //    let conn = state.db();
    //    let dao = MapAreaDao::new(&conn);

    //    dao.delete_by_id(query.id)?;

    //    Ok(ApiResult::success(()))
    // }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("MapArea", "private", base = "/map_area/v1")]
impl PrivateApi {}
