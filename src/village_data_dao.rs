//! Dao implementation for VillageData
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::query_source::joins::Join;
use diesel::sql_types;

use crate::{
    error::Error,
    models::{User, Village, VillageData},
    result::Result,
    schema::village_data,
    sqlutil::lower,
    types::{EntriesResult, Ops},
    util, ID,
};

#[doc(hidden)]
#[derive(Insertable)]
#[table_name = "village_data"]
pub struct NewVillageData<'a> {
    pub village_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    // pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub city_id: ID,
    pub meta: &'a Vec<&'a str>,
    pub district_id: ID,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
}

#[doc(hidden)]
pub struct UpdateVillageData<'a> {
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    // pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub city_id: Option<ID>,
    pub district_id: Option<ID>,
    pub meta: &'a Vec<&'a str>,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
}

/// Data Access Object for VillageData
#[derive(Dao)]
#[table_name = "village_data"]
pub struct VillageDataDao<'a> {
    db: &'a PgConnection,
}

impl<'a> VillageDataDao<'a> {
    /// Create new VillageData
    pub fn create(&self, data: &NewVillageData) -> Result<VillageData> {
        use crate::schema::village_data::{self, dsl};

        diesel::insert_into(village_data::table)
            .values(data)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Update data
    pub fn update(&self, village_id: ID, ops: Ops, data: &UpdateVillageData) -> Result<()> {
        use crate::schema::village_data::{self, dsl};

        // dbg!(&ops);

        let run_result = {
            match ops {
                Ops::Add => diesel::update(dsl::village_data.filter(dsl::village_id.eq(village_id)))
                    .set((
                        dsl::odp.eq(dsl::odp + data.odp),
                        dsl::pdp.eq(dsl::pdp + data.pdp),
                        dsl::cases.eq(dsl::cases + data.cases),
                        dsl::recovered.eq(dsl::recovered + data.recovered),
                        dsl::deaths.eq(dsl::deaths + data.deaths),
                        dsl::last_updated.eq(util::now()),
                        dsl::meta.eq(data.meta),
                        dsl::last_updated_by_id.eq(data.last_updated_by_id),
                        dsl::ppdwt.eq(dsl::ppdwt + data.ppdwt),
                        dsl::pptb.eq(dsl::pptb + data.pptb),
                        dsl::odpsp.eq(dsl::odpsp + data.odpsp),
                        dsl::pdps.eq(dsl::pdps + data.pdps),
                        dsl::pdpm.eq(dsl::pdpm + data.pdpm),
                        dsl::otg.eq(dsl::otg + data.otg),
                    ))
                    .execute(self.db),
                Ops::Subs => diesel::update(dsl::village_data.filter(dsl::village_id.eq(village_id)))
                    .set((
                        dsl::odp.eq(dsl::odp - data.odp),
                        dsl::pdp.eq(dsl::pdp - data.pdp),
                        dsl::cases.eq(dsl::cases - data.cases),
                        dsl::recovered.eq(dsl::recovered - data.recovered),
                        dsl::deaths.eq(dsl::deaths - data.deaths),
                        dsl::last_updated.eq(util::now()),
                        dsl::meta.eq(data.meta),
                        dsl::last_updated_by_id.eq(data.last_updated_by_id),
                        dsl::ppdwt.eq(dsl::ppdwt - data.ppdwt),
                        dsl::pptb.eq(dsl::pptb - data.pptb),
                        dsl::odpsp.eq(dsl::odpsp - data.odpsp),
                        dsl::pdps.eq(dsl::pdps - data.pdps),
                        dsl::pdpm.eq(dsl::pdpm - data.pdpm),
                        dsl::otg.eq(dsl::otg - data.otg),
                    ))
                    .execute(self.db),
                Ops::Set => diesel::update(dsl::village_data.filter(dsl::village_id.eq(village_id)))
                    .set((
                        dsl::odp.eq(data.odp),
                        dsl::pdp.eq(data.pdp),
                        dsl::cases.eq(data.cases),
                        dsl::recovered.eq(data.recovered),
                        dsl::deaths.eq(data.deaths),
                        dsl::last_updated.eq(util::now()),
                        dsl::meta.eq(data.meta),
                        dsl::last_updated_by_id.eq(data.last_updated_by_id),
                        dsl::ppdwt.eq(data.ppdwt),
                        dsl::pptb.eq(data.pptb),
                        dsl::odpsp.eq(data.odpsp),
                        dsl::pdps.eq(data.pdps),
                        dsl::pdpm.eq(data.pdpm),
                        dsl::otg.eq(data.otg),
                    ))
                    .execute(self.db),
            }
        };

        match run_result {
            Ok(updated) if updated == 0 => {
                // do insert
                let city_id = data
                    .city_id
                    .ok_or(Error::InvalidParameter("No city_id".to_string()))?;

                let district_id = data
                    .district_id
                    .ok_or(Error::InvalidParameter("No district_id".to_string()))?;

                self.create(&NewVillageData {
                    village_id,
                    district_id,
                    odp: data.odp,
                    pdp: data.pdp,
                    cases: data.cases,
                    recovered: data.recovered,
                    deaths: data.deaths,
                    last_updated_by_id: data.last_updated_by_id,
                    city_id,
                    meta: data.meta,
                    ppdwt: data.ppdwt,
                    pptb: data.pptb,
                    odpsp: data.odpsp,
                    pdps: data.pdps,
                    pdpm: data.pdpm,
                    otg: data.otg,
                })?;
            }
            Ok(_) => (),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => {
                // do insert
                let city_id = data
                    .city_id
                    .ok_or(Error::InvalidParameter("No city_id".to_string()))?;
                let district_id = data
                    .district_id
                    .ok_or(Error::InvalidParameter("No district_id".to_string()))?;

                self.create(&NewVillageData {
                    village_id,
                    district_id,
                    odp: data.odp,
                    pdp: data.pdp,
                    cases: data.cases,
                    recovered: data.recovered,
                    deaths: data.deaths,
                    last_updated_by_id: data.last_updated_by_id,
                    city_id,
                    meta: data.meta,
                    ppdwt: data.ppdwt,
                    pptb: data.pptb,
                    odpsp: data.odpsp,
                    pdps: data.pdps,
                    pdpm: data.pdpm,
                    otg: data.otg,
                })?;
            }
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }

    /// Search for specific village_data
    pub fn list(
        &self,
        city_id: ID,
        // query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<(VillageData, Village)>> {
        use crate::schema::village_data::{self, dsl};
        use crate::schema::villages::{self, dsl as dslv};
        // let like_clause = format!("%{}%", query);
        // let filterer: Box<dyn BoxableExpression<_, diesel::pg::Pg, SqlType = sql_types::Bool>> =
        //     Box::new(dsl::city_id.eq(&city_id));

        Ok(EntriesResult::new(
            dsl::village_data
                .inner_join(dslv::villages)
                .filter(dsl::city_id.eq(&city_id))
                .order(dsl::last_updated.desc())
                .offset(offset)
                .limit(limit)
                .load::<(VillageData, Village)>(self.db)?,
            dsl::village_data
                .filter(dsl::city_id.eq(&city_id))
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    /// Search for specific village_data
    pub fn search(
        &self,
        district_name: Option<&str>,
        // village_name: Option<&str>,
        query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<(VillageData, Village)>> {
        use crate::schema::village_data::{self, dsl};
        use crate::schema::villages::{self, dsl as dslv};

        let mut filterer: Box<dyn BoxableExpression<_, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        if !query.is_empty() {
            let like_clause = format!("%{}%", query.to_lowercase());
            filterer = Box::new(filterer.and(lower(dslv::name).like(like_clause)));
        }

        // if let Some(village_name) = village_name {
        //     filterer = Box::new(filterer.and(dsl::meta.contains(vec![format!("village={}", village_name)])));
        // }
        if let Some(district_name) = district_name {
            filterer =
                Box::new(filterer.and(dsl::meta.contains(vec![format!("district={}", district_name)])));
        }

        Ok(EntriesResult::new(
            dsl::village_data
                .inner_join(dslv::villages)
                .filter(&filterer)
                .offset(offset)
                .order(dsl::last_updated.desc())
                .limit(limit)
                .load::<(VillageData, Village)>(self.db)?,
            dsl::village_data
                .inner_join(dslv::villages)
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    /// Mendapatkan village data berdasarkan nama-nya.
    pub fn get_by_village_id(&self, id: ID) -> Result<Option<VillageData>> {
        use crate::schema::village_data::{self, dsl};
        match dsl::village_data.filter(dsl::id.eq(id)).first(self.db) {
            Err(diesel::result::Error::NotFound) => Ok(None),
            Ok(a) => Ok(Some(a)),
            Err(e) => Err(e.into()),
        }
    }
}
