//! Dao implementation for Feed
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

use crate::{models::Feed, result::Result, schema::feeds, sqlutil::lower, types::FeedKind, ID};

#[derive(Insertable)]
#[table_name = "feeds"]
struct NewFeed<'a> {
    pub creator_id: ID,
    pub creator_name: &'a str,
    pub loc: &'a str,
    pub kind: i16,
    pub text: &'a str,
    pub hashtags: &'a Vec<&'a str>,
    pub meta: &'a Vec<&'a str>,
}

/// Data Access Object for Feed
#[derive(Dao)]
#[table_name = "feeds"]
pub struct FeedDao<'a> {
    db: &'a PgConnection,
}

impl<'a> FeedDao<'a> {
    /// Create new Feed
    pub fn create(
        &self,
        creator_id: ID,
        creator_name: &'a str,
        loc: &'a str,
        kind: FeedKind,
        text: &'a str,
        hashtags: &'a Vec<&'a str>,
        meta: &'a Vec<&'a str>,
    ) -> Result<Feed> {
        use crate::schema::feeds::{self, dsl};

        diesel::insert_into(feeds::table)
            .values(&NewFeed {
                creator_id,
                creator_name,
                loc,
                kind: kind as i16,
                text,
                hashtags,
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Search for specific feeds
    pub fn search(
        &self,
        loc: Option<&str>,
        exclude_loc: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Feed>> {
        use crate::schema::feeds::{self, dsl};
        use diesel::dsl::not;
        
        let mut filterer: Box<dyn BoxableExpression<feeds::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0));

        if let Some(loc) = loc {
            filterer = Box::new(filterer.and(dsl::loc.like(format!("%{}%", loc))));
        } else if let Some(exclude_loc) = exclude_loc {
            filterer = Box::new(filterer.and(not(dsl::loc.like(format!("%{}%", exclude_loc)))));
        }

        dsl::feeds
            .filter(filterer)
            .order(dsl::ts.desc())
            .offset(offset)
            .limit(limit)
            .load(self.db)
            .map_err(From::from)
    }
}
