//! Dao implementation for City
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{models::City, result::Result, schema::cities, ID};

#[derive(Insertable)]
#[table_name = "cities"]
struct NewCity<'a> {
    pub name: &'a str,
    pub province: &'a str,
    pub country_code: &'a str,
    pub area_code: &'a str,
}

/// Data Access Object for City
#[derive(Dao)]
#[table_name = "cities"]
pub struct CityDao<'a> {
    db: &'a PgConnection,
}

impl<'a> CityDao<'a> {
    /// Create new City
    pub fn create(
        &self,
        name: &'a str,
        province: &'a str,
        country_code: &'a str,
        area_code: &'a str,
    ) -> Result<City> {
        use crate::schema::cities::{self, dsl};

        diesel::insert_into(cities::table)
            .values(&NewCity {
                name,
                province,
                country_code,
                area_code,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan citie berdasarkan kode area-nya.
    pub fn get_by_area_code(&self, code: &str) -> Result<Option<City>> {
        use crate::schema::cities::{self, dsl};

        match dsl::cities.filter(dsl::area_code.eq(code)).first(self.db) {
            Ok(a) => Ok(Some(a)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
