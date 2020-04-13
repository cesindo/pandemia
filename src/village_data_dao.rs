//! Dao implementation for VillageData
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    models::{User, VillageData},
    result::Result,
    schema::village_data,
    util, ID,
};

#[derive(Insertable)]
#[table_name = "village_data"]
struct NewVillageData {
    pub village_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    // pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
}

/// Data Access Object for VillageData
#[derive(Dao)]
#[table_name = "village_data"]
pub struct VillageDataDao<'a> {
    db: &'a PgConnection,
}

impl<'a> VillageDataDao<'a> {
    /// Create new VillageData
    pub fn create(
        &self,
        village_id: ID,
        odp: i32,
        pdp: i32,
        cases: i32,
        recovered: i32,
        deaths: i32,
        last_updated_by_id: ID,
    ) -> Result<VillageData> {
        use crate::schema::village_data::{self, dsl};

        diesel::insert_into(village_data::table)
            .values(&NewVillageData {
                village_id,
                odp,
                pdp,
                cases,
                recovered,
                deaths,
                last_updated_by_id,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Update data
    pub fn update(
        &self,
        village_id: ID,
        odp: i32,
        pdp: i32,
        cases: i32,
        recovered: i32,
        deaths: i32,
        updater: &User,
    ) -> Result<()> {
        use crate::schema::village_data::{self, dsl};
        match diesel::update(dsl::village_data.filter(dsl::village_id.eq(village_id)))
            .set((
                dsl::odp.eq(dsl::odp + odp),
                dsl::pdp.eq(dsl::pdp + pdp),
                dsl::cases.eq(dsl::cases + cases),
                dsl::recovered.eq(dsl::recovered + recovered),
                dsl::deaths.eq(dsl::deaths + deaths),
                dsl::last_updated.eq(util::now()),
                dsl::last_updated_by_id.eq(updater.id),
            ))
            .execute(self.db)
        {
            Ok(updated) if updated == 0 => {
                // do insert
                self.create(village_id, odp, pdp, cases, recovered, deaths, updater.id)?;
            }
            Ok(_) => (),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => {
                // do insert
                self.create(village_id, odp, pdp, cases, recovered, deaths, updater.id)?;
            }
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }
}
