//! Pandemia Types collection

/// Kind of account available in this system
pub enum AccountKind {
    /// Admin
    Admin = 0,

    /// User
    User = 1,
}

/// Healthy kind
pub enum HealthyKind {
    /// Health/sehat
    Health = 1,

    /// Sick/ada gejala
    Sick = 2,
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

/// Status sub reports
#[derive(Serialize, Copy, Clone, PartialEq, Eq)]
pub enum SubReportStatus {
    /// Orang Dalam Pemantauan
    ODP = 0,

    /// Pasien Dalam Pemantauan
    PDP = 1,

    /// Positif covid-19
    Positive = 2,

    /// Sembuh
    Recovered = 3,

    /// Sembuh
    Death = 4,

    /// Data untuk semua (hanya untuk query)
    All = -1,

    /// Unknown
    Unknown = 404,
}

impl Default for SubReportStatus {
    fn default() -> Self {
        SubReportStatus::Unknown
    }
}

impl std::fmt::Display for SubReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubReportStatus::ODP => write!(f, "ODP"),
            SubReportStatus::PDP => write!(f, "PDP"),
            SubReportStatus::Positive => write!(f, "POSITIVE"),
            SubReportStatus::Recovered => write!(f, "RECOVERED"),
            SubReportStatus::Death => write!(f, "DEATH"),
            SubReportStatus::All => write!(f, "ALL"),
            SubReportStatus::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl From<i32> for SubReportStatus {
    fn from(a: i32) -> Self {
        match a {
            0 => SubReportStatus::ODP,
            1 => SubReportStatus::PDP,
            2 => SubReportStatus::Positive,
            3 => SubReportStatus::Recovered,
            4 => SubReportStatus::Death,
            -1 => SubReportStatus::All,
            x => SubReportStatus::Unknown,
        }
    }
}

impl From<&str> for SubReportStatus {
    fn from(a: &str) -> Self {
        match a {
            "odp" => SubReportStatus::ODP,
            "pdp" => SubReportStatus::PDP,
            "positive" => SubReportStatus::Positive,
            "recovered" => SubReportStatus::Recovered,
            "death" => SubReportStatus::Death,
            "all" => SubReportStatus::All,
            _ => SubReportStatus::Unknown,
        }
    }
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
