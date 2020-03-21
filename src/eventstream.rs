//! Event stream and listener implementation for Pandemia

extern crate event_stream;

use diesel::prelude::*;

use self::event_stream::{EventDispatcher, EventDispatcherBuilder, EventListener};
use crate::event_handler;
use crate::{chrono, db, models::Record};

use std::{env, sync::Arc, thread::sleep, time::Duration};

/// Detax internal events
#[derive(Debug, Clone)]
pub enum Event {
    /// Event emited when service run on startup.
    Startup(),

    /// Event when new updates found from remote data sources
    /// params: 1: old record, 2: new record
    NewRecordUpdate(Option<Record>, Record), // @TODO(*): Add more events here
}

/// Pandemia event listener implemetation
#[derive(Clone)]
struct PandemiaEventListener {
    db: db::DbConnMan,
}

impl_event_listener!(PandemiaEventListener);

macro_rules! handle_event {
    ( $slf:ident, $handler:ident, $($params:expr,)* ) => {
        {
            let conn = $slf.db.get().expect("Cannot get db connection from pool");
            if let Err(e) = event_handler::$handler( $($params,)* &conn){
                error!("when {}: {}", stringify!($handler), e);
            }
        }
    };
    ($slf:ident, $handler:ident, $param1:expr) => {
        handle_event!($slf, $handler, $param1,)
    };
    ($slf:ident, $handler:ident, $param1:expr, $param2:expr ) => {
        handle_event!($slf, $handler, $param1, $param2,)
    };
    ($slf:ident, $handler:ident, $param1:expr, $param2:expr, $param3:expr ) => {
        handle_event!($slf, $handler, $param1, $param2, $param3,)
    };
    ($slf:ident, $handler:ident, $param1:expr, $param2:expr, $param3:expr, $param4:expr ) => {
        handle_event!($slf, $handler, $param1, $param2, $param3, $param4,)
    };
}

impl EventListener<Event> for PandemiaEventListener {
    fn dispatch(&self, event: &Event) {
        use self::Event::*;

        debug!("{:?} got event: {:?}", self, event);

        match event {
            Startup() => {
                debug!("on startup called");
            }
            NewRecordUpdate(old_record, new_record) => {
                handle_event!(self, new_record_update, old_record, new_record);
            } // _ => (),
        }
    }
}

impl std::fmt::Debug for PandemiaEventListener {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(out, "<PandemiaEventListener>")
    }
}

lazy_static! {

    /// Event dispatcher global var
    pub static ref EVENT_DISPATCHER:EventDispatcher<Event> = {
        let event_dispatcher = EventDispatcherBuilder::new()
        .add_listener(PandemiaEventListener::new())
        .build();

        event_dispatcher.start();
        event_dispatcher
    };
}

/// Emit event to the stream
pub fn emit(event: Event) {
    EVENT_DISPATCHER.emit(event)
}
