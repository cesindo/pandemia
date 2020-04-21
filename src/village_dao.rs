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
    pub district_name: &'a str,
    pub city: &'a str,
    pub province: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub meta: &'a Vec<&'a str>,
    pub city_id: ID,
    pub district_id: ID,
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
        district_name: &'a str,
        city: &'a str,
        province: &'a str,
        latitude: f64,
        longitude: f64,
        meta: &'a Vec<&'a str>,
        city_id: ID,
        district_id: ID,
    ) -> Result<Village> {
        use crate::schema::villages::{self, dsl};

        diesel::insert_into(villages::table)
            .values(&NewVillage {
                name,
                district_name,
                city,
                province,
                latitude,
                longitude,
                meta,
                city_id,
                district_id,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan village berdasarkan nama-nya.
    pub fn get_by_name_str(&self, province: &str, city: &str, name: &str) -> Result<Village> {
        use crate::schema::villages::{self, dsl};
        dsl::villages
            .filter(
                dsl::province
                    .eq(province)
                    .and(dsl::city.eq(city))
                    .and(dsl::name.eq(name)),
            )
            .first(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan village berdasarkan nama dan city_id.
    pub fn get_by_name_id(&self, city_id: ID, district_id: ID, name: &str) -> Result<Village> {
        use crate::schema::villages::{self, dsl};
        dsl::villages
            .filter(
                dsl::city_id
                    .eq(city_id)
                    .and(dsl::district_id.eq(district_id))
                    .and(lower(dsl::name).eq(name.to_lowercase())),
            )
            .first(self.db)
            .map_err(From::from)
    }

    /// Search for specific villages
    pub fn search(
        &self,
        query: &str,
        scope: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<Village>> {
        use crate::schema::villages::{self, dsl};

        let like_clause = format!("%{}%", query.to_lowercase());

        let mut filterer: Box<dyn BoxableExpression<villages::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        if let Some(scope) = scope {
            let s: Vec<&str> = scope.split("/").collect();
            if s.len() < 3 {
                fail!("Invalid scope");
            }
            let province = s[1];
            let city = s[2];
            filterer = Box::new(filterer.and(dsl::province.eq(province).and(dsl::city.eq(city))));

            if like_clause != "%%" {
                filterer = Box::new(
                    filterer.and(
                        lower(dsl::name)
                            .like(&like_clause)
                            .or(lower(dsl::district_name).like(&like_clause)),
                    ),
                );
            }
        } else if like_clause != "%%" {
            filterer = Box::new(
                filterer
                    .and(lower(dsl::name).like(&like_clause))
                    .or(lower(dsl::district_name).like(&like_clause))
                    .or(lower(dsl::city).like(&like_clause)),
            );
        }

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
