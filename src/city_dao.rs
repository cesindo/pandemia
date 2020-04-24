//! Dao implementation for City
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

use crate::{models::City, result::Result, schema::cities, sqlutil::lower, ID, types::EntriesResult};

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

    /// Mendapatkan city berdasarkan nama-nya.
    pub fn get_by_name(&self, province: &str, name: &str) -> Result<City> {
        use crate::schema::cities::{self, dsl};
        dsl::cities
            .filter(
                lower(dsl::name)
                    .eq(name.to_lowercase())
                    .and(lower(dsl::province).eq(province.to_lowercase())),
            )
            .first(self.db)
            .map_err(From::from)
    }

    /// Search for specific cities
    pub fn search(&self, query: &str, offset: i64, limit: i64) -> Result<EntriesResult<City>> {
        use crate::schema::cities::{self, dsl};

        let like_clause = format!("%{}%", query);

        let mut filterer: Box<dyn BoxableExpression<cities::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        filterer = Box::new(filterer.and(dsl::name.like(&like_clause)));

        Ok(EntriesResult::new(
            dsl::cities
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .load::<City>(self.db)?,
            dsl::cities
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
