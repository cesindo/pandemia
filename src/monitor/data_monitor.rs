//! Data monitor
//!

use diesel::prelude::*;
use reqwest;
use select::document::Document;
use select::node::Data::*;
use select::predicate::{Attr, Name};
use std::{fs::File, io::BufReader};

use crate::{
    db,
    error::Error,
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
        if let Err(e) = DataMonitor::check_worldometers() {
            error!("check_worldometers. e {}", e);
        }
        Ok(())
    }

    /// Check data from https://www.worldometers.info/coronavirus/
    pub fn check_worldometers() -> Result<()> {
        debug!("Checking Worldometers...");
        let resp = reqwest::get("https://www.worldometers.info/coronavirus/")?;
        Document::from_read(resp)?
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
            .for_each(|a| {
                println!("[Worldometers] data collected: {:?}", a);
            });

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
