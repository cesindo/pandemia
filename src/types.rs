//! Pandemia Types collection

/// Notification kind or types
#[derive(Serialize, Copy, Clone)]
pub enum NotifKind {
    /// reserved
    Reserved
}

/// Location kind
pub enum LocKind {
    /// Global
    Global = 0,
    /// Continent
    Continent = 1,
    /// Country
    Country = 2,
    /// Province
    Province = 3,
    /// City
    City = 4
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

/// Record diff result
pub struct RecordDiff {
    /// +/- new cases
    pub new_cases: i32,
    /// +/- new deaths
    pub new_deaths: i32,
    /// +/- new recovered
    pub new_recovered: i32,
    /// +/- new critical
    pub new_critical: i32,
}

/// Jenis-jenis feed
/// ini bertipe i16 atau SMALLINT pada SQL.
pub enum FeedKind {
    /// Feed are created by system
    SystemFeed = 0,

    /// Merupakan feed yang memberikan informasi pengumuman secara global,
    /// feed ini tidak ada creatornya atau creatornya adalah system.
    Announcement = 1,

    /// When new deaths record found
    NewDeaths = 2,

    /// When new recovered record found
    NewRecovered = 3,

    /// Info 
    Info = 6
}

impl From<i16> for FeedKind {
    fn from(i: i16) -> Self {
        match i {
            0 => FeedKind::SystemFeed,
            1 => FeedKind::Announcement,
            2 => FeedKind::NewDeaths,
            3 => FeedKind::NewRecovered,
            x => panic!("Unknown feed kind number: {}", x),
        }
    }
}
