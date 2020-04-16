//! Dao implementation for District
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{models::District, result::Result, schema::districts, sqlutil::lower, ID};

#[derive(Insertable)]
#[table_name = "districts"]
struct NewDistrict<'a> {
    pub name: &'a str,
    pub city_id: ID,
    pub meta: &'a Vec<&'a str>,
}

/// Data Access Object for District
#[derive(Dao)]
#[table_name = "districts"]
pub struct DistrictDao<'a> {
    db: &'a PgConnection,
}

impl<'a> DistrictDao<'a> {
    /// Create new District
    pub fn create(&self, name: &'a str, city_id: ID, meta: &'a Vec<&'a str>) -> Result<District> {
        use crate::schema::districts::{self, dsl};

        diesel::insert_into(districts::table)
            .values(&NewDistrict { name, city_id, meta })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan district berdasarkan nama-nya.
    pub fn get_by_name(&self, city_id: ID, name: &str) -> Result<District> {
        use crate::schema::districts::{self, dsl};
        dsl::districts
            .filter(
                lower(dsl::name)
                    .eq(name.to_lowercase())
                    .and(dsl::city_id.eq(city_id)),
            )
            .first(self.db)
            .map_err(From::from)
    }
}
