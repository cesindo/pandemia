//! Dao implementation for Village
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

use crate::{models::Village, result::Result, schema::villages, sqlutil::lower, types::EntriesResult, ID};

#[derive(Insertable)]
#[table_name = "villages"]
struct NewVillage<'a> {
    pub name: &'a str,
    pub sub_district: &'a str,
    pub city: &'a str,
    pub province: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub meta: &'a Vec<&'a str>,
}

/// Data Access Object for Village
#[derive(Dao)]
#[table_name = "villages"]
pub struct VillageDao<'a> {
    db: &'a PgConnection,
}

impl<'a> VillageDao<'a> {
    /// Create new Village
    pub fn create(
        &self,
        name: &'a str,
        sub_district: &'a str,
        city: &'a str,
        province: &'a str,
        latitude: f64,
        longitude: f64,
        meta: &'a Vec<&'a str>,
    ) -> Result<Village> {
        use crate::schema::villages::{self, dsl};

        diesel::insert_into(villages::table)
            .values(&NewVillage {
                name,
                sub_district,
                city,
                province,
                latitude,
                longitude,
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan village berdasarkan nama-nya.
    pub fn get_by_name(&self, name: &str) -> Result<Village> {
        use crate::schema::villages::{self, dsl};
        dsl::villages
            .filter(dsl::name.eq(name))
            .first(self.db)
            .map_err(From::from)
    }

    /// Search for specific villages
    pub fn search(&self, query: &str, offset: i64, limit: i64) -> Result<EntriesResult<Village>> {
        use crate::schema::villages::{self, dsl};

        let like_clause = format!("%{}%", query);

        let mut filterer: Box<dyn BoxableExpression<villages::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        filterer = Box::new(
            filterer
                .and(lower(dsl::name).like(&like_clause))
                .or(lower(dsl::sub_district).like(&like_clause))
                .or(lower(dsl::city).like(&like_clause)),
        );

        Ok(EntriesResult::new(
            dsl::villages
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .load::<Village>(self.db)?,
            dsl::villages
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
