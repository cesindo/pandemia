//! Pandemia Types collection

/// Notification kind or types
#[derive(Serialize, Copy, Clone)]
pub enum NotifKind {
    /// when new record found notification
    NewRecordFound,
}

/// Entries result type
#[derive(Serialize, Deserialize)]
pub struct EntriesResult<T> {
    /// list of entries
    pub entries: Vec<T>,
    /// count of entries
    pub count: i64,
}

impl<T> EntriesResult<T> {
    /// Create new instance of EntriesResult
    pub fn new(entries: Vec<T>, count: i64) -> Self {
        Self { entries, count }
    }
}
