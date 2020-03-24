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
        // dapatkan total global cases
        let main_counter_numbers = doc
            .find(Attr("class", "maincounter-number"))
            .map(|a| a.text().trim().to_string())
            .collect::<Vec<String>>();

        match &main_counter_numbers.as_slice() {
            &[total_cases, total_deaths, recovered] => {
                let total_cases = total_cases.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let total_deaths = total_deaths.replace(",", "").trim().parse::<i32>().unwrap_or(0);
                let recovered = recovered.replace(",", "").trim().parse::<i32>().unwrap_or(0);

                let latest_data = dao.get_latest_records(Some("global"), 0, 1)?.pop();

                if let Some(latest_data) = latest_data {
                    if latest_data.total_cases != total_cases {
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
                    )?;
                }
            }
            x => {
                warn!("got invalid number of columns, expected {}, got {}", 3, x.len());
            }
        }

        let collected: Vec<Vec<String>> = doc
            .find(Attr("id", "main_table_countries_today"))
            .flat_map(|a| {
                a.find(Name("tr"))
                    .map(|a| {
                        let childs = a.children();
                        // let fname = childs.next()?.next()?;
                        // // dbg!(fname.inner_html().trim());
                        // // fname.find(Name("a")).map(|a| a.inner_html().trim())
                        // let country_name = fname.text();
                        // // dbg!(&country_name.trim());
                        // if country_name.trim() == "Indonesia" {
                        //     Some(
                        //         childs
                        //             .filter(|b| b.name().is_some())
                        //             // .map(|b| format!("{:?}", b) )
                        //             .map(|b| b.text().trim().to_owned())
                        //             .collect::<Vec<String>>(),
                        //     )
                        // } else {
                        //     None
                        // }

                        childs
                            .filter(|b| b.name().is_some())
                            // .map(|b| format!("{:?}", b) )
                            .map(|b| b.text().trim().to_owned())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>()
            })
            .collect();
        // .for_each(|a| {
        //     println!("[Worldometers] data collected: {:?}", a);

        // });
        if !collected.is_empty() {
            println!("[Worldometers] data collected: {:?}", collected.len());
            let mut has_indonesia = false;

            for a in &collected {
                match &a.as_slice() {
                    &[country_name, total_cases, new_cases, total_deaths, new_deaths, recovered, active_cases, critical_cases, cases_to_pop] =>
                    {
                        // sementara ini Indonesia saja dulu
                        if country_name != "Indonesia" {
                            continue;
                        }

                        has_indonesia = true;

                        // get latest record to diff
                        let latest_record = dao.get_latest_records(Some(country_name), 0, 1)?.pop();

                        if let Some(latest_record) = latest_record {
                            if latest_record.total_cases != total_cases.parse::<i32>().unwrap_or(0) {
                                let new_record = dao.create(
                                    &country_name,
                                    LocKind::Country,
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
                            // very new
                            let new_record = dao.create(
                                &country_name,
                                LocKind::Country,
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

                            debug!("new record saved.");

                            eventstream::emit(NewRecordUpdate(None, new_record.clone()));
                        }
                    }
                    _ => (),
                }
            }

            if !has_indonesia {
                warn!("{} data found, but no Indonesia found", collected.len());
                warn!("DUMP:\n{:?}", collected);
            }
        } else {
            warn!("Collection is empty!");
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
            for i in 0..60 {
                util::sleep(1000);
            }
            // util::sleep(1000);
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
