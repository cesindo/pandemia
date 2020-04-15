//! Dao implementation for ReportNote
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

use crate::{models::ReportNote, result::Result, schema::report_notes, types::EntriesResult, ID};

#[derive(Insertable)]
#[table_name = "report_notes"]
struct NewReportNote<'a> {
    pub title: &'a str,
    pub notes: &'a str,
    pub creator_id: ID,
    pub creator_name: &'a str,
    pub city_id: ID,
    pub meta: &'a Vec<&'a str>,
}

/// Data Access Object for ReportNote
#[derive(Dao)]
#[table_name = "report_notes"]
pub struct ReportNoteDao<'a> {
    db: &'a PgConnection,
}

impl<'a> ReportNoteDao<'a> {
    /// Create new ReportNote
    pub fn create(
        &self,
        title: &'a str,
        notes: &'a str,
        creator_id: ID,
        creator_name: &'a str,
        city_id: ID,
        meta: &'a Vec<&'a str>,
    ) -> Result<ReportNote> {
        use crate::schema::report_notes::{self, dsl};

        diesel::insert_into(report_notes::table)
            .values(&NewReportNote {
                title,
                notes,
                creator_id,
                creator_name,
                city_id,
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Update state
    pub fn update(&self, id: ID, approved: bool) -> Result<()> {
        use crate::schema::report_notes::{self, dsl};

        diesel::update(dsl::report_notes.filter(dsl::id.eq(id)))
            .set(dsl::approved.eq(approved))
            .execute(self.db)?;

        Ok(())
    }

    /// Search for specific report_notes
    pub fn search(
        &self,
        city_id: ID,
        query: &str,
        state: &str,
        meta_contains: Vec<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<ReportNote>> {
        use crate::schema::report_notes::{self, dsl};

        let like_clause = format!("%{}%", query);

        let mut filterer: Box<dyn BoxableExpression<report_notes::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        if city_id > 0 {
            filterer = Box::new(filterer.and(dsl::city_id.eq(city_id)));
        }

        if state == "approved" {
            filterer = Box::new(filterer.and(dsl::approved.eq(true)));
        } else if state == "unapproved" {
            filterer = Box::new(filterer.and(dsl::approved.eq(false)));
        }

        filterer = Box::new(filterer.and(dsl::notes.like(&like_clause)));

        if !meta_contains.is_empty() {
            filterer = Box::new(filterer.and(dsl::meta.contains(meta_contains)));
        }

        Ok(EntriesResult::new(
            dsl::report_notes
                .filter(&filterer)
                .offset(offset)
                .order(dsl::ts.desc())
                .limit(limit)
                .load::<ReportNote>(self.db)?,
            dsl::report_notes
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
