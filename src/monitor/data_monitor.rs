//! Data monitor
//!

use diesel::prelude::*;
use regex::Regex;
use reqwest;
use select::document::Document;
use select::node::Data::*;
use select::predicate::{Attr, Class, Name};
use std::{fs::File, io::BufReader};

use crate::{
    dao::RecordDao,
    db,
    error::Error,
    eventstream::{self, Event::NewRecordUpdate},
    // event_handler::FCM,
    // models::{User, Comment, HasID, MonitoredData},
    monitor::{Monitor, PandemiaMonitor},
    record_dao::MutateRecord,
    // push_notif_handler::{FCMHandler, FCMPayloadData},
    result::Result,
    types::LocKind,
    util,
    ID,
};

use std::{
    fmt,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

/// Untuk serialize json dari server
#[derive(Debug, Serialize, Deserialize)]
struct ResultItem {
    /// Field ID
    #[serde(rename = "FID")]
    pub fid: i64,

    /// Provinsi
    #[serde(rename = "Provinsi")]
    pub province: String,

    /// Jumlah Kasus Meninggal
    #[serde(rename = "Kasus_Meni")]
    pub total_deaths: i32,

    /// Jumlah Kasus Positif
    #[serde(rename = "Kasus_Posi")]
    pub active_cases: i32,

    /// Jumlah Kasus Sembuh
    #[serde(rename = "Kasus_Semb")]
    pub total_recovered: i32,
}

/// Untuk serialize json object dari server kawalcorona.com
#[derive(Debug, Serialize, Deserialize)]
struct ResultObject {
    /// Field attributes
    pub attributes: ResultItem,
}

/// Data monitoring
pub struct DataMonitor {
    _started: bool,
    _tx: Option<Sender<bool>>,
}

unsafe impl Sync for DataMonitor {}
unsafe impl Send for DataMonitor {}

impl DataMonitor {
    /// Create DataMonitor new instance
    pub fn new() -> PandemiaMonitor {
        Mutex::new(Box::new(Self {
            _started: false,
            _tx: None,
        }))
    }
}

impl fmt::Display for DataMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DataMonitor")
    }
}

impl DataMonitor {
    /// Datas checker
    pub fn check_data(conn: &PgConnection) -> Result<()> {
        if let Err(e) = DataMonitor::check_worldometers(conn) {
            error!("check_worldometers. e {}", e);
        }

        if let Err(e) = DataMonitor::check_indonesian_provinces(conn) {
            error!("check kawalcorona.com, e {}", e);
        }
        Ok(())
    }

    /// Check data from https://api.kawalcorona.com/indonesia/provinsi/
    pub fn check_indonesian_provinces(conn: &PgConnection) -> Result<()> {
        debug!("Fetching data from kawalcorona.com ...");
        let dao = RecordDao::new(conn);
        let resp = reqwest::get("https://api.kawalcorona.com/indonesia/provinsi/");
        let items: Vec<ResultObject> = serde_json::from_str(&resp?.text()?)?;
        for data in &items {
            let item = &data.attributes;
            // beberapa provinsi ini di-exclude karena data
            // akan diambil terpisah dari sumber resmi
            if item.province.to_lowercase() == "jawa tengah" {
                continue;
            }

            let loc_path = format!("/Indonesia/{}", item.province);

            let total_cases: i32 = item.active_cases + item.total_deaths + item.total_recovered;
            let latest_data = dao.get_latest_record_one(&loc_path).ok();

            debug!(
                "Fetching data for Prov. {}, with total cases: {}",
                &item.province, &total_cases
            );

            if let Some(latest_data) = latest_data {
                if latest_data.total_cases != total_cases {
                    let new_record = dao.create(
                        &MutateRecord {
                            loc: &item.province,
                            loc_kind: LocKind::Province as i16,
                            total_cases,
                            total_deaths: item.total_deaths,
                            total_recovered: item.total_recovered,
                            active_cases: item.active_cases,
                            critical_cases: 0,
                            meta: vec!["loc_scope:indonesia"],
                            loc_path: &loc_path,
                            ..Default::default()
                        },
                        false,
                    )?;

                    debug!("new record from Prov. {} saved.", &item.province);

                    let diff = new_record.diff(&latest_data);

                    if diff.new_cases > 0
                        || diff.new_deaths > 0
                        || diff.new_recovered > 0
                        || diff.new_critical > 0
                    {
                        eventstream::emit(NewRecordUpdate(Some(latest_data.clone()), new_record.clone()));
                    }
                }
            } else {
                dao.create(
                    &MutateRecord {
                        loc: &item.province,
                        loc_kind: LocKind::Province as i16,
                        total_cases,
                        total_deaths: item.total_deaths,
                        total_recovered: item.total_recovered,
                        active_cases: item.active_cases,
                        critical_cases: 0,
                        meta: vec!["loc_scope:indonesia"],
                        loc_path: &loc_path,
                        ..Default::default()
                    },
                    false,
                )?;
            }
        }

        if let Err(e) = Self::get_jatengprov(conn) {
            error!("Cannot get data from jatengprov.go.id");
        }

        Ok(())
    }

    fn num_only<'a>(s: &'a str) -> std::borrow::Cow<'a, str> {
        let re = Regex::new("[^0-9]").unwrap();
        re.replace_all(s, "")
    }

    /// Get data from official Jateng Province site https://corona.jatengprov.go.id/
    pub fn get_jatengprov(conn: &PgConnection) -> Result<()> {
        let resp = reqwest::get("https://corona.jatengprov.go.id/")?;
        let doc = Document::from_read(resp)?;

        // dapatkan total cases global
        let counter_numbers = doc
            .find(Class("font-counter"))
            .map(|a| a.text().trim().to_string())
            .flat_map(|a| {
                Self::num_only(&a)
                    .split(' ')
                    .flat_map(|a| a.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
                    .first()
                    .cloned()
            })
            .collect::<Vec<i32>>();

        dbg!(&counter_numbers);

        if counter_numbers.len() != 6 {
            return Err(Error::InvalidParameter(
                "Bad data from server, html structure changed".to_string(),
            ));
        }

        let name = "Jawa Tengah";

        let dao = RecordDao::new(conn);

        let loc_path = format!("/Indonesia/{}", name);

        let prev_record = dao.get_latest_record_one(&loc_path).ok();

        if let Some(prev_record) = prev_record {
            match &counter_numbers[0..6] {
                &[active_cases, positive, recovered, deaths, odp, pdp] => {
                    if prev_record.total_cases != active_cases {
                        let new_record = dao.create(
                            &MutateRecord {
                                loc: name,
                                loc_kind: LocKind::Province as i16,
                                total_cases: active_cases,
                                total_deaths: deaths,
                                total_recovered: recovered,
                                active_cases: 0,
                                critical_cases: 0,
                                meta: vec!["loc_scope:indonesia"],
                                loc_path: &loc_path,
                                ..Default::default()
                            },
                            false,
                        )?;
                        debug!("new record from Prov. {} saved.", name);
                        let diff = new_record.diff(&prev_record);
                        if diff.new_cases > 0
                            || diff.new_deaths > 0
                            || diff.new_recovered > 0
                            || diff.new_critical > 0
                        {
                            eventstream::emit(NewRecordUpdate(Some(prev_record.clone()), new_record.clone()));
                        }
                    }
                }
                _ => (),
            }
        } else {
            match &counter_numbers[0..6] {
                &[active_cases, positive, recovered, deaths, odp, pdp] => {
                    dao.create(
                        &MutateRecord {
                            loc: name,
                            loc_kind: LocKind::Province as i16,
                            total_cases: active_cases,
                            total_deaths: deaths,
                            total_recovered: recovered,
                            active_cases: 0,
                            critical_cases: 0,
                            meta: vec!["loc_scope:indonesia"],
                            ..Default::default()
                        },
                        false,
                    )?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    /// Check data from https://www.worldometers.info/coronavirus/
    pub fn check_worldometers(conn: &PgConnection) -> Result<()> {
        debug!("Fetching data from Worldometers...");
        let dao = RecordDao::new(conn);
        let resp = reqwest::get("https://www.worldometers.info/coronavirus/");
        // dbg!(&resp);
        let doc = Document::from_read(resp?)?;
        // dapatkan total cases global
        let main_counter_numbers = doc
            .find(Class("maincounter-number"))
            .map(|a| a.text().trim().to_string())
            .collect::<Vec<String>>();

        match &main_counter_numbers.as_slice() {
            &[total_cases, total_deaths, recovered] => {
                let total_cases = total_cases.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let total_deaths = total_deaths.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let recovered = recovered.replace(",", "").trim().parse::<i32>().unwrap_or(0);

                let loc_path = "/global".to_owned();

                let latest_record = dao.get_latest_record_one(&loc_path).ok();

                if let Some(latest_record) = latest_record {
                    if latest_record.total_cases != total_cases {
                        let new_record = dao.create(
                            // "global",
                            // LocKind::Global,
                            // total_cases,
                            // total_deaths,
                            // recovered,
                            // 0,
                            // 0,
                            &MutateRecord {
                                loc: "global",
                                loc_kind: LocKind::Global as i16,
                                total_cases,
                                total_deaths,
                                total_recovered: recovered,
                                active_cases: 0,
                                critical_cases: 0,
                                meta: vec![],
                                loc_path: &loc_path,
                                ..Default::default()
                            },
                            false,
                        )?;
                    }
                } else {
                    dao.create(
                        // "global",
                        // LocKind::Global,
                        // total_cases,
                        // total_deaths,
                        // recovered,
                        // 0,
                        // 0,
                        // vec![],
                        &MutateRecord {
                            loc: "global",
                            loc_kind: LocKind::Global as i16,
                            total_cases,
                            total_deaths,
                            total_recovered: recovered,
                            active_cases: 0,
                            critical_cases: 0,
                            meta: vec![],
                            loc_path: &loc_path,
                            ..Default::default()
                        },
                        false,
                    )?;
                }
            }
            x => {
                warn!("got invalid number of columns, expected {}, got {}", 3, x.len());
            }
        }

        let resp = reqwest::get("https://www.worldometers.info/coronavirus/country/indonesia/");
        // dbg!(&resp);
        let doc = Document::from_read(resp?)?;
        // dapatkan total cases global
        let main_counter_numbers = doc
            .find(Attr("class", "maincounter-number"))
            .map(|a| a.text().trim().to_string())
            .collect::<Vec<String>>();

        // dapatkan total cases nasional
        let main_counter_numbers = doc
            .find(Attr("class", "maincounter-number"))
            .map(|a| a.text().trim().to_string())
            .collect::<Vec<String>>();

        match &main_counter_numbers.as_slice() {
            &[total_cases, total_deaths, recovered] => {
                let total_cases = total_cases.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let total_deaths = total_deaths.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let recovered = recovered.replace(",", "").trim().parse::<i32>().unwrap_or(0);

                let loc_path = "/Indonesia".to_string();

                let latest_record = dao.get_latest_record_one(&loc_path).ok();

                if let Some(latest_record) = latest_record {
                    if latest_record.total_cases != total_cases {
                        let new_record = dao.create(
                            // "Indonesia",
                            // LocKind::Country,
                            // total_cases,
                            // total_deaths,
                            // recovered,
                            // 0,
                            // 0,
                            // vec!["loc_scope:indonesia"],
                            &MutateRecord {
                                loc: "Indonesia",
                                loc_kind: LocKind::Country as i16,
                                total_cases,
                                total_deaths,
                                total_recovered: recovered,
                                active_cases: 0,
                                critical_cases: 0,
                                meta: vec!["loc_scope:indonesia"],
                                loc_path: &loc_path,
                                ..Default::default()
                            },
                            false,
                        )?;

                        debug!("new record saved.");

                        let diff = new_record.diff(&latest_record);

                        if diff.new_cases > 0
                            || diff.new_deaths > 0
                            || diff.new_recovered > 0
                            || diff.new_critical > 0
                        {
                            eventstream::emit(NewRecordUpdate(
                                Some(latest_record.clone()),
                                new_record.clone(),
                            ));
                        }
                    }
                } else {
                    dao.create(
                        // "Indonesia",
                        // LocKind::Country,
                        // total_cases,
                        // total_deaths,
                        // recovered,
                        // 0,
                        // 0,
                        // vec!["loc_scope:indonesia"],
                        &MutateRecord {
                            loc: "Indonesia",
                            loc_kind: LocKind::Country as i16,
                            total_cases,
                            total_deaths,
                            total_recovered: recovered,
                            active_cases: 0,
                            critical_cases: 0,
                            meta: vec!["loc_scope:indonesia"],
                            loc_path: &loc_path,
                            ..Default::default()
                        },
                        false,
                    )?;
                }
            }
            x => {
                warn!("got invalid number of columns, expected {}, got {}", 3, x.len());
            }
        }

        Ok(())
    }
}

impl Monitor for DataMonitor {
    fn start(&mut self) {
        let (tx, rx) = channel();
        self._tx = Some(tx);
        self._started = true;
        thread::spawn(move || loop {
            for i in 0..(60 * 30) {
                // setiap setengah jam
                util::sleep(1000);
            }
            // util::sleep(1000);
            // debug!("[DataMonitor] monitor checking...");

            let th = thread::spawn(move || {
                let cm = db::clone();
                let conn = cm.get().unwrap();

                if let Err(e) = DataMonitor::check_data(&conn) {
                    error!("Data monitor check_data error: {}", e);
                }
            });

            let _ = th.join();

            if rx.try_recv().ok() == Some(true) {
                debug!("[DataMonitor] down.");
                break;
            }
        });
    }

    fn stop(&mut self) {
        self._started = false;
        self._tx.as_ref().map(|tx| tx.send(true));
    }
}
