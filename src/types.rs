//! Pandemia Types collection

/// Kind of account available in this system
pub enum AccountKind {
    /// Admin
    Admin = 0,

    /// User
    User = 1,
}

/// Notification kind or types
#[derive(Serialize, Copy, Clone)]
pub enum NotifKind {
    /// Merupakan notif yang memberikan informasi pengumuman secara global,
    /// notif ini tidak ada creatornya atau creatornya adalah system.
    Announcement = 1,

    /// When new cases found
    NewCases = 2,

    /// When new deaths record found
    NewDeaths = 3,

    /// When new recovered record found
    NewRecovered = 4,

    /// Info
    Info = 6,
}

/// Location kind
#[derive(Copy, Clone)]
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
    City = 4,
    /// Unknown
    Unknown = 10,
}

impl From<i16> for LocKind {
    fn from(d: i16) -> Self {
        use LocKind::*;
        match d {
            0 => Global,
            1 => Continent,
            2 => Country,
            3 => Province,
            4 => City,
            x => {
                error!("Unknown loc kind code: {}", x);
                Unknown
            }
        }
    }
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
#[derive(Debug)]
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

    /// When new cases found
    NewCases = 2,

    /// When new deaths record found
    NewDeaths = 3,

    /// When new recovered record found
    NewRecovered = 4,

    /// Info
    Info = 6,
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

/// Map marker kind
pub enum MapMarkerKind {
    /// Unknown type
    Unknown = 0,

    /// Pandemic information type
    PandemicInfo = 1,

    /// Sick
    Sick = 2,

    /// Fasilitas kesehatan (Faskes) bertipe Rumah Sakit
    Hospital = 3,
}

impl From<i16> for MapMarkerKind {
    fn from(i: i16) -> Self {
        use MapMarkerKind::*;
        match i {
            1 => PandemicInfo,
            2 => Sick,
            3 => Hospital,
            _ => Unknown,
        }
    }
}

impl From<MapMarkerKind> for i16 {
    fn from(a: MapMarkerKind) -> Self {
        a as i16
    }
}
