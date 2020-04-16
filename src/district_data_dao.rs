//! Dao implementation for DistrictData
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::query_source::joins::Join;
use diesel::sql_types;

use crate::{
    models::{District, DistrictData, User},
    result::Result,
    schema::district_data,
    types::EntriesResult,
    util, ID,
};

#[derive(Insertable)]
#[table_name = "district_data"]
struct NewDistrictData<'a> {
    pub district_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub city_id: ID,
    pub meta: &'a Vec<&'a str>,
}

/// Data Access Object for DistrictData
#[derive(Dao)]
#[table_name = "district_data"]
pub struct DistrictDataDao<'a> {
    db: &'a PgConnection,
}

impl<'a> DistrictDataDao<'a> {
    /// Create new DistrictData
    pub fn create(
        &self,
        district_id: ID,
        odp: i32,
        pdp: i32,
        cases: i32,
        recovered: i32,
        deaths: i32,
        last_updated_by_id: ID,
        city_id: ID,
        meta: &'a Vec<&'a str>,
    ) -> Result<DistrictData> {
        use crate::schema::district_data::{self, dsl};

        diesel::insert_into(district_data::table)
            .values(&NewDistrictData {
                district_id,
                odp,
                pdp,
                cases,
                recovered,
                deaths,
                last_updated: util::now(),
                last_updated_by_id,
                city_id,
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Update data
    pub fn update(
        &self,
        district_id: ID,
        odp: i32,
        pdp: i32,
        cases: i32,
        recovered: i32,
        deaths: i32,
        updater_id: ID,
        city_id: ID,
        meta: &Vec<&str>,
    ) -> Result<()> {
        use crate::schema::district_data::{self, dsl};
        match diesel::update(dsl::district_data.filter(dsl::district_id.eq(district_id)))
            .set((
                dsl::odp.eq(dsl::odp + odp),
                dsl::pdp.eq(dsl::pdp + pdp),
                dsl::cases.eq(dsl::cases + cases),
                dsl::recovered.eq(dsl::recovered + recovered),
                dsl::deaths.eq(dsl::deaths + deaths),
                dsl::last_updated.eq(util::now()),
                dsl::meta.eq(meta),
                dsl::last_updated_by_id.eq(updater_id),
            ))
            .execute(self.db)
        {
            Ok(updated) if updated == 0 => {
                // do insert
                self.create(
                    district_id,
                    odp,
                    pdp,
                    cases,
                    recovered,
                    deaths,
                    updater_id,
                    city_id,
                    meta,
                )?;
            }
            Ok(_) => (),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => {
                // do insert
                self.create(
                    district_id,
                    odp,
                    pdp,
                    cases,
                    recovered,
                    deaths,
                    updater_id,
                    city_id,
                    meta,
                )?;
            }
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }

    /// Search for specific district_data
    pub fn list(
        &self,
        city_id: ID,
        // query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<(DistrictData, District)>> {
        use crate::schema::district_data::{self, dsl};
        use crate::schema::districts::{self, dsl as dslv};
        // let like_clause = format!("%{}%", query);
        // let filterer: Box<dyn BoxableExpression<_, diesel::pg::Pg, SqlType = sql_types::Bool>> =
        //     Box::new(dsl::city_id.eq(&city_id));

        Ok(EntriesResult::new(
            dsl::district_data
                .inner_join(dslv::districts)
                .filter(dsl::city_id.eq(&city_id))
                .offset(offset)
                .limit(limit)
                .load::<(DistrictData, District)>(self.db)?,
            dsl::district_data
                .filter(dsl::city_id.eq(&city_id))
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
