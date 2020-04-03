//! Dao implementation for Record
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{dsl::any, sql_types};

use crate::{
    models::Record, result::Result, schema::records, sqlutil::lower, types::EntriesResult, types::LocKind, ID,
};

/// This model structure modeled after data from https://www.worldometers.info/coronavirus/
#[derive(Insertable)]
#[table_name = "records"]
struct NewRecord<'a> {
    pub loc: &'a str,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub meta: &'a Vec<&'a str>,
    pub latest: bool,
}

/// Data Access Object for Record
#[derive(Dao)]
#[table_name = "records"]
pub struct RecordDao<'a> {
    db: &'a PgConnection,
}

impl<'a> RecordDao<'a> {
    /// Create new Record
    pub fn create(
        &self,
        loc: &'a str,
        loc_kind: LocKind,
        total_cases: i32,
        total_deaths: i32,
        total_recovered: i32,
        active_cases: i32,
        critical_cases: i32,
        meta: &'a Vec<&'a str>,
        no_tx: bool,
    ) -> Result<Record> {
        if no_tx {
            self.do_update(
                loc,
                loc_kind,
                total_cases,
                total_deaths,
                total_recovered,
                active_cases,
                critical_cases,
                meta,
            )
        } else {
            self.db.build_transaction().read_write().run::<_, _, _>(|| {
                self.do_update(
                    loc,
                    loc_kind,
                    total_cases,
                    total_deaths,
                    total_recovered,
                    active_cases,
                    critical_cases,
                    meta,
                )
            })
        }
    }

    fn do_update(
        &self,
        loc: &'a str,
        loc_kind: LocKind,
        total_cases: i32,
        total_deaths: i32,
        total_recovered: i32,
        active_cases: i32,
        critical_cases: i32,
        meta: &'a Vec<&'a str>,
    ) -> Result<Record> {
        use crate::schema::records::{self, dsl};

        // reset dulu yang ada flag latest-nya ke false
        diesel::update(
            dsl::records.filter(
                dsl::loc
                    .eq(loc)
                    .and(dsl::loc_kind.eq(loc_kind as i16))
                    .and(dsl::latest.eq(true)),
            ),
        )
        .set(dsl::latest.eq(false))
        .execute(self.db)?;

        // tambahkan record baru dengan latest=true
        diesel::insert_into(records::table)
            .values(&NewRecord {
                loc,
                loc_kind: loc_kind as i16,
                total_cases,
                total_deaths,
                total_recovered,
                active_cases,
                critical_cases,
                meta,
                latest: true,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Get latest records
    pub fn get_latest_records(&self, locs: Vec<&str>, offset: i64, limit: i64) -> Result<Vec<Record>> {
        use crate::schema::records::dsl;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        if !locs.is_empty() {
            dsl::records
                .filter(dsl::loc.eq(any(locs)).and(dsl::latest.eq(true)))
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
    pub fn get_record_history(&self, loc: &str, offset: i64, limit: i64) -> Result<Vec<Record>> {
        use crate::schema::records::dsl;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        dsl::records
            .filter(dsl::loc.eq(loc))
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
