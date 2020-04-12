//! Untuk memonitor perubahan dan melakukan perubahan apabila diperlukan
//! monitor ini jalan di thread lain atau terpisah dengan thread utama
//! karena akan melakukan pengecheckan secara berkala.
//!
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{dsl::not, expression::dsl::sql, sql_query, sql_types};

use crate::{db, models, result::Result, util};

use std::{
    fmt,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub mod data_monitor;
pub use data_monitor::DataMonitor;

/// Base type for PandemiaMonitor
pub type PandemiaMonitor = Mutex<Box<dyn Monitor>>;

/// Abstraksi untuk sistem monitor
pub trait Monitor: Send + Sync + fmt::Display {
    /// Jalankan monitor
    fn start(&mut self);

    /// Berhentikan monitor
    fn stop(&mut self);
}

// ------------ MONITOR CONTROLLER ---------------

lazy_static! {
    static ref MONITORS: Vec<PandemiaMonitor> = vec![DataMonitor::new()];
}

/// Run all monitors
pub fn start_monitors() {
    debug!("Starting monitors...");

    // {
    //     // for debugging purpose
    //     let conn = db::clone().get().unwrap();
    //     let _ = DataMonitor::get_jatengprov(&conn);
    // }

    for monitor in MONITORS.iter() {
        let mut monitor = monitor.lock().unwrap();
        debug!("Starting `{}`...", monitor);
        monitor.start();
    }
}

/// Stop all monitors
pub fn stop_monitors() {
    for monitor in MONITORS.iter() {
        let mut monitor = monitor.lock().unwrap();
        monitor.stop();
    }
}
