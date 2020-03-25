//! Main event handler module
use chrono::prelude::*;
use diesel::prelude::*;

use crate::{api::types, models, result::Result, token, util, ID};

mod data_event_handler;

pub use crate::push_notif_handler::{FCMHandler, FCMPayloadData};
pub use data_event_handler::*;

lazy_static! {
    /// FCM push handler
    pub static ref FCM:FCMHandler = {
        FCMHandler::new()
    };
}
