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
    // event_handler::FCM,
    // models::{User, Comment, HasID, MonitoredData},
    monitor::{Monitor, PandemiaMonitor},
    // push_notif_handler::{FCMHandler, FCMPayloadData},
    result::Result,
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
        Ok(())
    }

    /// Check data from https://www.worldometers.info/coronavirus/
    pub fn check_worldometers(conn: &PgConnection) -> Result<()> {
        debug!("Checking Worldometers...");
        let resp = reqwest::get("https://www.worldometers.info/coronavirus/")?;
        let collected: Vec<Vec<String>> = Document::from_read(resp)?
            .find(Attr("id", "main_table_countries"))
            .map(|a| {
                a.find(Name("tr"))
                    .flat_map(|a| {
                        let mut childs = a.children();
                        let fname = childs.next()?.next()?;
                        if fname.inner_html().trim() == "Indonesia" {
                            Some(
                                childs
                                    .filter(|b| b.name().is_some())
                                    // .map(|b| format!("{:?}", b) )
                                    .map(|b| b.inner_html().trim().to_owned())
                                    .collect::<Vec<String>>()
                                    .join(", "),
                            )
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .collect();
        // .for_each(|a| {
        //     println!("[Worldometers] data collected: {:?}", a);

        // });
        if !collected.is_empty() {
            println!("[Worldometers] data collected: {:?}", collected.len());
            let dao = RecordDao::new(conn);

            for a in collected {
                match &a.as_slice() {
                    &[country_name, total_cases, new_cases, total_deaths, new_deaths, recovered, active_cases, critical_cases, cases_to_pop] =>
                    {
                        // sementara ini Indonesia saja dulu
                        if country_name != "Indonesia" {
                            continue;
                        }

                        // get latest record to diff
                        let latest_record = dao.get_latest_records(country_name, 0, 1)?.pop();

                        let new_record = dao.create(
                            &country_name,
                            1,
                            total_cases.parse().unwrap_or(0),
                            // new_cases.parse().unwrap_or(0),
                            total_deaths.parse().unwrap_or(0),
                            // new_deaths.parse().unwrap_or(0),
                            recovered.parse().unwrap_or(0),
                            active_cases.parse().unwrap_or(0),
                            critical_cases.parse().unwrap_or(0),
                            cases_to_pop.parse().unwrap_or(0.0),
                            &vec![],
                        )?;

                        if let Some(latest_record) = latest_record {
                            let new_cases = new_record.total_cases - latest_record.total_cases;
                            let new_deaths = new_record.total_deaths - latest_record.total_deaths;
                            let new_recovered = new_record.total_recovered - latest_record.total_recovered;
                            let new_critical = new_record.critical_cases - latest_record.critical_cases;

                            if new_cases > 0 || new_deaths > 0 || new_recovered > 0 || new_critical > 0 {
                                eventstream::emit(NewRecordUpdate(latest_record.clone(), new_record.clone()));
                            }
                        }
                    }
                    _ => (),
                }
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
            // for i in 0..60 {
            //     util::sleep(1000);
            // }
            util::sleep(1000);
            // debug!("[DataMonitor] monitor checking...");

            let cm = db::clone();
            let conn = cm.get().unwrap();

            if let Err(e) = DataMonitor::check_data(&conn) {
                error!("Data monitor check_data error: {}", e);
            }

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
