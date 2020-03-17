//! DAO re-exports

use diesel::sql_types;

pub use crate::admin_dao::AdminDao;

sql_function!(
    /// To lowerize sql column value typely
    fn lower(x: sql_types::Text) -> sql_types::Text);

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

