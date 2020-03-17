//! Event stream and listener implementation for Pandemia

extern crate event_stream;

use diesel::prelude::*;

use self::event_stream::{EventDispatcher, EventDispatcherBuilder, EventListener};
use crate::{chrono, db};

use std::{env, sync::Arc, thread::sleep, time::Duration};

/// Detax internal events
#[derive(Debug, Clone)]
pub enum Event {
    /// Event emited when service run on startup.
    Startup(),

    // @TODO(*): Add more events here
}

/// Pandemia event listener implemetation
#[derive(Clone)]
struct PandemiaEventListener {
    db: db::DbConnMan,
}

impl_event_listener!(PandemiaEventListener);

impl EventListener<Event> for PandemiaEventListener {
    fn dispatch(&self, event: &Event) {
        use self::Event::*;

        debug!("{:?} got event: {:?}", self, event);

        match event {
            Startup() => {
                debug!("on startup called");
            }
            // _ => (),
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

