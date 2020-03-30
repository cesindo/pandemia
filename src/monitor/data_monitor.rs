//! Data monitor
//!

use diesel::prelude::*;
use reqwest;
use select::document::Document;
use select::node::Data::*;
use select::predicate::{Attr, Name};
use std::{fs::File, io::BufReader};

use crate::{
    dao::RecordDao,
    db,
    error::Error,
    eventstream::{self, Event::NewRecordUpdate},
    models::{ResultItem, ResultObject},
    // event_handler::FCM,
    // models::{User, Comment, HasID, MonitoredData},
    monitor::{Monitor, PandemiaMonitor},
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
            let total_cases: i32 = item.active_cases + item.total_deaths + item.total_recovered;
            let latest_data = dao.get_latest_records(Some(&item.province), 0, 1)?.pop();

            debug!(
                "Fetching data for Prov. {}, with total cases: {}",
                &item.province, &total_cases
            );

            if let Some(latest_data) = latest_data {
                if latest_data.total_cases != total_cases {
                    let new_record = dao.create(
                        &item.province,
                        LocKind::Province,
                        total_cases,
                        item.total_deaths,
                        item.total_recovered,
                        item.active_cases,
                        0,
                        0.0,
                        &vec![],
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
                    &item.province,
                    LocKind::Province,
                    total_cases,
                    item.total_deaths,
                    item.total_recovered,
                    item.active_cases,
                    0,
                    0.0,
                    &vec![],
                    false,
                )?;
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
            .find(Attr("class", "maincounter-number"))
            .map(|a| a.text().trim().to_string())
            .collect::<Vec<String>>();

        match &main_counter_numbers.as_slice() {
            &[total_cases, total_deaths, recovered] => {
                let total_cases = total_cases.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let total_deaths = total_deaths.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let recovered = recovered.replace(",", "").trim().parse::<i32>().unwrap_or(0);

                let latest_record = dao.get_latest_records(Some("global"), 0, 1)?.pop();

                if let Some(latest_record) = latest_record {
                    if latest_record.total_cases != total_cases {
                        let new_record = dao.create(
                            "global",
                            LocKind::Global,
                            total_cases,
                            total_deaths,
                            recovered,
                            0,
                            0,
                            0.0,
                            &vec![],
                            false,
                        )?;
                    }
                } else {
                    dao.create(
                        "global",
                        LocKind::Global,
                        total_cases,
                        total_deaths,
                        recovered,
                        0,
                        0,
                        0.0,
                        &vec![],
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

                let latest_record = dao.get_latest_records(Some("Indonesia"), 0, 1)?.pop();

                if let Some(latest_record) = latest_record {
                    if latest_record.total_cases != total_cases {
                        let new_record = dao.create(
                            "Indonesia",
                            LocKind::Country,
                            total_cases,
                            total_deaths,
                            recovered,
                            0,
                            0,
                            0.0,
                            &vec![],
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
                        "Indonesia",
                        LocKind::Country,
                        total_cases,
                        total_deaths,
                        recovered,
                        0,
                        0,
                        0.0,
                        &vec![],
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
