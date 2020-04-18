//! Dao implementation for DistrictData
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::query_source::joins::Join;
use diesel::sql_query;
use diesel::sql_types;
use diesel::sql_types::BigInt;

use crate::{
    error::Error,
    models::{District, DistrictData, User},
    result::Result,
    schema::district_data,
    types::{EntriesResult, Ops},
    util, ID,
};

#[doc(hidden)]
#[derive(Insertable)]
#[table_name = "district_data"]
pub struct NewDistrictData<'a> {
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

    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
}

#[doc(hidden)]
pub struct UpdateDistrictData<'a> {
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    // pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub city_id: ID,
    pub meta: &'a Vec<&'a str>,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
}

/// Data Access Object for DistrictData
#[derive(Dao)]
#[table_name = "district_data"]
pub struct DistrictDataDao<'a> {
    db: &'a PgConnection,
}

impl<'a> DistrictDataDao<'a> {
    /// Create new DistrictData
    pub fn create(&self, data: &NewDistrictData) -> Result<DistrictData> {
        use crate::schema::district_data::{self, dsl};

        diesel::insert_into(district_data::table)
            .values(data)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Update data
    pub fn update(&self, district_id: ID, ops: Ops, data: &UpdateDistrictData) -> Result<()> {
        use crate::schema::district_data::{self, dsl};
        let result = match ops {
            Ops::Add => diesel::update(dsl::district_data.filter(dsl::district_id.eq(district_id)))
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
            Ops::Subs => diesel::update(dsl::district_data.filter(dsl::district_id.eq(district_id)))
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
            Ops::Set => diesel::update(dsl::district_data.filter(dsl::district_id.eq(district_id)))
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
        };
        match result {
            Ok(updated) if updated == 0 => {
                // do insert
                self.create(&NewDistrictData {
                    district_id,
                    odp: data.odp,
                    pdp: data.pdp,
                    cases: data.cases,
                    recovered: data.recovered,
                    deaths: data.deaths,
                    last_updated: util::now(),
                    last_updated_by_id: data.last_updated_by_id,
                    city_id: data.city_id,
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
                self.create(&NewDistrictData {
                    district_id,
                    odp: data.odp,
                    pdp: data.pdp,
                    cases: data.cases,
                    recovered: data.recovered,
                    deaths: data.deaths,
                    last_updated: util::now(),
                    last_updated_by_id: data.last_updated_by_id,
                    city_id: data.city_id,
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
                .order(dsl::last_updated.desc())
                .offset(offset)
                .limit(limit)
                .load::<(DistrictData, District)>(self.db)?,
            dsl::district_data
                .filter(dsl::city_id.eq(&city_id))
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    /// Update recalculate
    pub fn recalculate(&self, city_id: ID, district_id: ID, village_id: ID, updater_id: ID) -> Result<()> {
        use crate::schema::district_data::{self, dsl};

        // rekalkulasi data district diambil dari village
        let mut entry: Vec<Entry> = sql_query(&format!(
            "SELECT \
            SUM(odp) as odp, \
            SUM(pdp) as pdp, \
            SUM(cases) as positive, \
            SUM(recovered) as recovered, \
            SUM(deaths) as deaths, \
            SUM(ppdwt) as ppdwt, \
            SUM(pptb) as pptb, \
            SUM(odpsp) as odpsp, \
            SUM(pdps) as pdps, \
            SUM(pdpm) as pdpm, \
            SUM(otg) as otg \
            FROM village_data WHERE district_id={}",
            district_id
        ))
        .load(self.db)
        .map_err(Error::from)?;

        let entry: Entry = entry
            .pop()
            .ok_or(Error::InternalError(format_err!("Cannot sum data")))?;

        // diesel::update(dsl::district_data.filter(dsl::id.eq(district_id)))
        //     .set((
        //         dsl::odp.eq(entry.odp),
        //         dsl::pdp.eq(entry.pdp)
        //     ))
        //     .execute(self.db)
        //     .map_err(Error::from)?;

        self.update(
            district_id,
            Ops::Set,
            &UpdateDistrictData {
                odp: entry.odp as i32,
                pdp: entry.pdp as i32,
                cases: entry.positive as i32,
                recovered: entry.recovered as i32,
                deaths: entry.deaths as i32,
                last_updated_by_id: updater_id,
                city_id,
                meta: &vec![],
                ppdwt: entry.ppdwt as i32,
                pptb: entry.pptb as i32,
                odpsp: entry.odpsp as i32,
                pdps: entry.pdps as i32,
                pdpm: entry.pdpm as i32,
                otg: entry.otg as i32,
            },
        )?;

        Ok(())
    }
}

#[derive(QueryableByName)]
struct Entry {
    #[sql_type = "BigInt"]
    odp: i64,
    #[sql_type = "BigInt"]
    pdp: i64,
    #[sql_type = "BigInt"]
    positive: i64,
    #[sql_type = "BigInt"]
    recovered: i64,
    #[sql_type = "BigInt"]
    deaths: i64,

    #[sql_type = "BigInt"]
    ppdwt: i64,
    #[sql_type = "BigInt"]
    pptb: i64,
    #[sql_type = "BigInt"]
    odpsp: i64,
    #[sql_type = "BigInt"]
    pdps: i64,
    #[sql_type = "BigInt"]
    pdpm: i64,
    #[sql_type = "BigInt"]
    otg: i64,
}
