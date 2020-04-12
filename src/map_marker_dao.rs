//! Dao implementation for MapMarker
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

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
    pub fn get_by_name(&self, name: &str, meta_contains: Vec<&str>) -> Result<Option<MapMarker>> {
        use crate::schema::map_markers::{self, dsl};

        let mut filterer: Box<dyn BoxableExpression<map_markers::table, _, SqlType = sql_types::Bool>> =
            Box::new(lower(dsl::name).eq(name.to_lowercase()));

        if !meta_contains.is_empty() {
            filterer = Box::new(filterer.and(dsl::meta.contains(meta_contains)));
        }

        match dsl::map_markers.filter(filterer).first::<MapMarker>(self.db) {
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

    // /// Search for specific map_markers
    // pub fn search(&self, query: &str, offset: i64, limit: i64) -> Result<EntriesResult<MapMarker>> {
    //     use crate::schema::map_markers::{self, dsl};

    //     let like_clause = format!("%{}%", query);

    //     let mut filterer: Box<dyn BoxableExpression<map_markers::table, _, SqlType = sql_types::Bool>> =
    //         Box::new(dsl::id.ne(0));

    //     filterer = Box::new(filterer.and(dsl::name.like(&like_clause)));

    //     Ok(EntriesResult::new(
    //           dsl::map_markers
    //                 .filter(&filterer)
    //                 .offset(offset)
    //                 .limit(limit)
    //                 .load::<MapMarker>(self.db)?,
    //           dsl::map_markers
    //                 .filter(filterer)
    //                 .select(diesel::dsl::count(dsl::id))
    //                 .first(self.db)?,
    //     ))
    // }
}
