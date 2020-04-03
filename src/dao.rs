//! DAO re-exports

use diesel::sql_types;

pub use crate::admin_dao::AdminDao;
pub use crate::feed_dao::FeedDao;
pub use crate::map_marker_dao::MapMarkerDao;
pub use crate::notif_dao::NotifDao;
pub use crate::record_dao::RecordDao;
pub use crate::user_dao::UserDao;

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
