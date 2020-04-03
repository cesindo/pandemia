//! Dao implementation for MapMarker
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    error::Error, models::MapMarker, result::Result, schema::map_markers, sqlutil::lower,
    types::MapMarkerKind, ID,
};

#[derive(Insertable)]
#[table_name = "map_markers"]
struct NewMapMarker<'a> {
    pub name: &'a str,
    pub info: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub kind: i16,
    pub meta: &'a Vec<&'a str>,
}

/// Data Access Object for MapMarker
#[derive(Dao)]
#[table_name = "map_markers"]
pub struct MapMarkerDao<'a> {
    db: &'a PgConnection,
}

impl<'a> MapMarkerDao<'a> {
    /// Create new MapMarker
    pub fn create(
        &self,
        name: &'a str,
        info: &'a str,
        latitude: f64,
        longitude: f64,
        kind: MapMarkerKind,
        meta: &'a Vec<&'a str>,
    ) -> Result<MapMarker> {
        use crate::schema::map_markers::{self, dsl};

        diesel::insert_into(map_markers::table)
            .values(&NewMapMarker {
                name,
                info,
                latitude,
                longitude,
                kind: kind.into(),
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan map_marker berdasarkan nama-nya.
    pub fn get_by_name(&self, name: &str) -> Result<Option<MapMarker>> {
        use crate::schema::map_markers::{self, dsl};
        match dsl::map_markers
            .filter(lower(dsl::name).eq(name.to_lowercase()))
            .first::<MapMarker>(self.db)
        {
            Ok(marker) => Ok(Some(marker)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Update meta data
    pub fn update_meta(&self, id: ID, meta: Vec<String>) -> Result<()> {
        use crate::schema::map_markers::{self, dsl};
        diesel::update(dsl::map_markers.filter(dsl::id.eq(id)))
            .set(dsl::meta.eq(meta))
            .execute(self.db)
            .map_err(Error::from)?;
        Ok(())
    }
}
