//! DAO untuk Sub Report
//!

use crate::{
    models::SubReport,
    result::Result,
    schema::sub_reports,
    sqlutil::lower,
    types::{EntriesResult, SubReportStatus},
    util, ID,
};
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::{dsl::any, sql_types};

#[derive(Insertable)]
#[table_name = "sub_reports"]
struct NewSubReport<'a> {
    pub creator_id: i64,
    pub creator_name: &'a str,
    pub full_name: &'a str,
    pub age: i32,
    pub residence_address: &'a str,
    pub gender: &'a str,
    pub arrival_address: &'a str,
    pub arrival_date: NaiveDate,
    pub healthy: i32,
    pub desc: &'a str,
    pub status: i32,
    pub meta: &'a Vec<&'a str>,
    pub ts: NaiveDateTime,
}

/// Data Access Object for SubReport
#[derive(Dao)]
#[table_name = "sub_reports"]
pub struct SubReportDao<'a> {
    db: &'a PgConnection,
}

impl<'a> SubReportDao<'a> {
    /// Create new SubReport
    pub fn create(
        &self,
        creator_id: i64,
        creator_name: &'a str,
        full_name: &'a str,
        age: i32,
        residence_address: &'a str,
        gender: &'a str,
        arrival_address: &'a str,
        arrival_date: NaiveDate,
        healthy: i32,
        desc: &'a str,
        status: i32,
        meta: &'a Vec<&'a str>,
    ) -> Result<SubReport> {
        use crate::schema::sub_reports::{self, dsl};

        diesel::insert_into(sub_reports::table)
            .values(&NewSubReport {
                creator_id,
                creator_name,
                full_name,
                age,
                residence_address,
                gender,
                arrival_address,
                arrival_date,
                healthy,
                desc,
                status,
                meta,
                ts: util::now(),
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Search for specific sub_reports
    pub fn search(
        &self,
        creator_id: i64,
        status: i32,
        query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<SubReport>> {
        use crate::schema::sub_reports::{self, dsl};
        let mut filterer: Box<dyn BoxableExpression<sub_reports::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        if query != "" {
            let like_clause = format!("%{}%", query).to_lowercase();
            filterer = Box::new(
                filterer.and(
                    lower(dsl::full_name)
                        .like(like_clause)
                        .and(dsl::creator_id.eq(creator_id)),
                ),
            );
        } else {
            filterer = Box::new(filterer.and(dsl::creator_id.eq(creator_id)));
        }

        filterer = Box::new(filterer.and(dsl::status.eq(status)));

        Ok(EntriesResult::new(
            dsl::sub_reports
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .order(dsl::ts.desc())
                .load::<SubReport>(self.db)?,
            dsl::sub_reports
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
