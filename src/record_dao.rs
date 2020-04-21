//! Dao implementation for Record
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{dsl::any, sql_types};

use crate::{
    models::Record,
    result::Result,
    schema::records,
    sqlutil::{array_append, lower},
    types::EntriesResult,
    types::{LocKind, Ops},
    ID,
};

/// This model structure modeled after data from https://www.worldometers.info/coronavirus/
#[doc(hidden)]
#[derive(Insertable, Default)]
#[table_name = "records"]
pub struct MutateRecord<'a> {
    pub loc: &'a str,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub meta: Vec<&'a str>,
    pub latest: bool,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odp: i32,
    pub odpsp: i32,
    pub pdp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,

    pub loc_path: &'a str,
}

// impl<'a> Default for MutateRecord<'a> {
//     fn default() -> Self {
//         Self {
//             meta: &'a vec![],
//             ..Default::default()
//         }
//     }
// }

// pub struct MutateMeta {
//     pub ops:Ops,
//     pub data:String
// }

/// Data Access Object for Record
#[derive(Dao)]
#[table_name = "records"]
pub struct RecordDao<'a> {
    db: &'a PgConnection,
}

impl<'a> RecordDao<'a> {
    /// Create new Record
    pub fn create(&self, nr: &MutateRecord, no_tx: bool) -> Result<Record> {
        if no_tx {
            self.do_update(nr)
        } else {
            self.db
                .build_transaction()
                .read_write()
                .run::<_, _, _>(|| self.do_update(nr))
        }
    }

    fn do_update(&self, nr: &MutateRecord) -> Result<Record> {
        use crate::schema::records::{self, dsl};

        // reset dulu yang ada flag latest-nya ke false
        diesel::update(
            dsl::records.filter(
                dsl::loc_path
                    .eq(nr.loc_path)
                    .and(dsl::loc_kind.eq(nr.loc_kind))
                    .and(dsl::latest.eq(true)),
            ),
        )
        .set(dsl::latest.eq(false))
        .execute(self.db)?;

        // tambahkan record baru dengan latest=true
        diesel::insert_into(records::table)
            .values(&MutateRecord {
                loc: nr.loc,
                loc_kind: nr.loc_kind,
                total_cases: nr.total_cases,
                total_deaths: nr.total_deaths,
                total_recovered: nr.total_recovered,
                active_cases: nr.active_cases,
                critical_cases: nr.critical_cases,
                meta: nr.meta.clone(),

                ppdwt: nr.ppdwt,
                pptb: nr.pptb,
                odp: nr.odp,
                pdp: nr.pdp,
                odpsp: nr.odpsp,
                pdps: nr.pdps,
                pdpm: nr.pdpm,
                otg: nr.otg,

                latest: true,
                loc_path: &nr.loc_path,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Get one latest record
    pub fn get_latest_record_one(&self, loc_path: &String) -> Result<Record> {
        use crate::schema::records::dsl;

        dsl::records
            .filter(dsl::loc_path.eq(loc_path).and(dsl::latest.eq(true)))
            .first(self.db)
            .map_err(From::from)
    }

    /// Get latest records
    pub fn get_latest_records(
        &self,
        loc_paths: &Vec<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Record>> {
        use crate::schema::records::dsl;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        if !loc_paths.is_empty() {
            let mut filterer: Box<dyn BoxableExpression<records::table, _, SqlType = sql_types::Bool>> =
                Box::new(dsl::latest.eq(true));

            let mut filterer2: Box<dyn BoxableExpression<records::table, _, SqlType = sql_types::Bool>> =
                Box::new(dsl::id.ne(0));

            for loc_path in loc_paths {
                let like_clause = format!("{}%", loc_path);
                filterer2 = Box::new(filterer2.or(dsl::loc_path.like(like_clause)));
            }
            filterer = Box::new(filterer.and(filterer2));

            dsl::records
                // .filter(dsl::loc.like(any(locs)).and(dsl::latest.eq(true)))
                .filter(filterer.and(dsl::latest.eq(true)))
                .order(dsl::last_updated.desc())
                .offset(offset)
                .limit(limit)
                .load(self.db)
                .map_err(From::from)
        } else {
            dsl::records
                .order(dsl::last_updated.desc())
                .offset(offset)
                .limit(limit)
                .load(self.db)
                .map_err(From::from)
        }
    }

    /// Get all records by loc
    pub fn get_record_history(&self, loc_path: &str, offset: i64, limit: i64) -> Result<Vec<Record>> {
        use crate::schema::records::dsl;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        let like_clause = format!("{}%", loc_path);

        dsl::records
            .filter(dsl::loc.like(like_clause))
            .order(dsl::last_updated.desc())
            .offset(offset)
            .limit(limit)
            .load(self.db)
            .map_err(From::from)
    }

    /// Search for specific records only take the latest one for each location
    pub fn search(&self, query: &str, offset: i64, limit: i64) -> Result<EntriesResult<Record>> {
        use crate::schema::records::{self, dsl};

        // deprecated, telah digantikan dengan pre-set column latest lebih murah dan optimal
        // select * from (select *, rank() OVER (PARTITION BY loc order by last_updated desc) from records) as d where d.rank=1;

        let like_clause = format!("%{}%", query.to_lowercase());
        let mut filterer: Box<dyn BoxableExpression<records::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0).and(dsl::latest.eq(true)));

        filterer = Box::new(filterer.and(lower(dsl::loc).like(&like_clause)));

        Ok(EntriesResult::new(
            dsl::records
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .order(dsl::last_updated.desc())
                .load::<Record>(self.db)?,
            dsl::records
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
