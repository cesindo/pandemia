//! DAO re-exports

use diesel::sql_types;

pub use crate::admin_dao::AdminDao;
pub use crate::city_dao::CityDao;
pub use crate::dao::journal::Logs;
pub use crate::feed_dao::FeedDao;
pub use crate::map_marker_dao::MapMarkerDao;
pub use crate::notif_dao::NotifDao;
pub use crate::record_dao::RecordDao;
pub use crate::sub_report_dao::SubReportDao;
pub use crate::user_dao::UserDao;
pub use crate::village_dao::VillageDao;
pub use crate::village_data_dao::VillageDataDao;

/// Search result type from DAO (not rest API)
pub struct EntriesResult<T> {
    /// list of entry
    pub entries: Vec<T>,
    /// entry count found
    pub count: i64,
}

impl<T> EntriesResult<T> {
    /// Create new entries result type
    pub fn new(entries: Vec<T>, count: i64) -> Self {
        Self { entries, count }
    }
}

mod journal {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;
    use diesel::sql_types;

    use crate::{models, result::Result, schema::logs, types::EntriesResult, ID};

    #[derive(Insertable)]
    #[table_name = "logs"]
    struct NewLog<'a> {
        activity: &'a str,
        initiator_id: ID,
        meta: &'a Vec<String>,
    }

    /// For internal logging
    pub struct Logs<'a> {
        /// db connection
        db: &'a PgConnection,
    }

    impl<'a> Logs<'a> {
        /// Create new logs instance
        pub fn new(conn: &'a PgConnection) -> Self {
            Self { db: conn }
        }

        /// Write log journal
        pub fn write(&self, activity: &str, initiator_id: ID) {
            use crate::schema::logs::{self, dsl};
            if let Err(e) = diesel::insert_into(logs::table)
                .values(&NewLog {
                    activity,
                    initiator_id,
                    meta: &vec![],
                })
                .execute(self.db)
            {
                error!("Cannot wirte log journal {}", e)
            }
        }

        /// Search for specific logs
        pub fn search(&self, query: &str, offset: i64, limit: i64) -> Result<EntriesResult<models::Log>> {
            use crate::schema::logs::{self, dsl};
            let like_clause = format!("%{}%", query);
            let mut filterer: Box<dyn BoxableExpression<logs::table, _, SqlType = sql_types::Bool>> =
                Box::new(dsl::id.ne(0));
            filterer = Box::new(filterer.and(dsl::activity.like(&like_clause)));
            Ok(EntriesResult::new(
                dsl::logs
                    .filter(&filterer)
                    .order(dsl::ts.desc())
                    .offset(offset)
                    .limit(limit)
                    .load::<models::Log>(self.db)?,
                dsl::logs
                    .filter(filterer)
                    .select(diesel::dsl::count(dsl::id))
                    .first(self.db)?,
            ))
        }
    }
}
