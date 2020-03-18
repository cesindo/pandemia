//! Event handler for data records
use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    api::types,
    event_handler::FCM,
    eventstream::{self, Event::*},
    models::{Record, User},
    notif_sender::send_notif,
    push_notif_handler::{FCMHandler, FCMPayloadData},
    result::Result,
    token,
    types::NotifKind,
    user_dao, util, ID,
};

/// Event handler when new record updates found
pub fn new_record_update(old_record: &Record, new_record: &Record, conn: &PgConnection) -> Result<()> {
    Ok(())
}
