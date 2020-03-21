//! Dao implementation for Record
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{models::Record, result::Result, schema::records, types::LocKind, ID};

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
    pub cases_to_pop: f64,
    pub meta: &'a Vec<&'a str>,
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
        cases_to_pop: f64,
        meta: &'a Vec<&'a str>,
    ) -> Result<Record> {
        use crate::schema::records::{self, dsl};

        diesel::insert_into(records::table)
            .values(&NewRecord {
                loc,
                loc_kind: loc_kind as i16,
                total_cases,
                total_deaths,
                total_recovered,
                active_cases,
                critical_cases,
                cases_to_pop,
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Get stock histories based on Record
    pub fn get_latest_records(&self, loc: Option<&str>, offset: i64, limit: i64) -> Result<Vec<Record>> {
        use crate::schema::records::dsl;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        if let Some(loc) = loc {
            dsl::records
                .filter(dsl::loc.eq(loc))
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
}
