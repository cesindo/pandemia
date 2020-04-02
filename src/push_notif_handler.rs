//! Fungsi yang meng-handle push notifikasi menggunakan FCM.
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

use crate::{
    result::Result,
    sqlutil::lower,
    types::{LocKind, NotifKind},
    ID,
};

use fcm::{Client, MessageBuilder, NotificationBuilder, Priority};
use futures::{future::lazy, Future};
use tokio_core::reactor::Core;

use std::env;

/// FCM payload data.
pub struct FCMPayloadData<'a> {
    /// Receiver location.
    pub receiver_loc: &'a str,
    /// Receiver location kind.
    pub receiver_loc_kind: LocKind,
    /// Target id.
    pub target_id: ID,
    /// Target item
    pub item: &'a str,
    /// Kind data.
    pub kind: NotifKind,
    /// Title data.
    pub title: &'a str,
    /// Message data.
    pub message: &'a str,
    /// Creation time.
    pub created: NaiveDateTime,
    /// FCM click action.
    pub click_action: &'a str,
}

/// FCM payload data.
#[derive(Serialize)]
struct FCMPayloadDataWire<'a> {
    /// Receiver location.
    pub receiver_loc: &'a str,
    /// Target id.
    pub target_id: ID,
    /// Target item
    pub item: &'a str,
    /// Kind data.
    pub kind: i32,
    /// Title data.
    pub title: &'a str,
    /// Message data.
    pub message: &'a str,
    /// Creation time.
    pub created: NaiveDateTime,
    /// FCM click action.
    pub click_action: &'a str,
}

// impl<'a> From<FCMPayloadData<'a>> for FCMPayloadDataWire<'a> {
//     fn from(a: FCMPayloadData<'a>) -> Self {
//         Self {
//             receiver_loc: a.receiver_loc,
//             target_id: a.target_id,
//             item: a.item,
//             kind: a.kind as i32,
//             title: a.title,
//             message: a.message,
//             created: a.created,
//             click_action: a.click_action,
//         }
//     }
// }

/// FCM push notification handler.
pub struct FCMHandler {
    server_key: String,
    client: Client,
}

// lazy_static! {
//     static ref REACTOR:Core = Core::new().expect("cannot get new Core");
// }

impl FCMHandler {
    /// Add push notification handler.
    pub fn new() -> FCMHandler {
        FCMHandler {
            server_key: env::var("FCM_SERVER_KEY").expect("No FCM_SERVER_KEY env var"),
            client: fcm::Client::new().unwrap(),
        }
    }

    /// Get app id from user connect.
    // fn get_user_app_id(&self, user_id: ID, conn: &PgConnection) -> Result<String> {
    //     use crate::schema::user_connect::{self, dsl};
    //     dsl::user_connect
    //         .select(dsl::app_id)
    //         .filter(dsl::user_id.eq(user_id))
    //         .first(conn)
    //         .map_err(From::from)
    // }

    /// Get app ids from user connect
    fn get_user_app_ids(
        &self,
        conn: &PgConnection,
        location: &str,
        loc_kind: LocKind,
    ) -> Result<Vec<String>> {
        use crate::schema::user_connect::{self, dsl as dsl_uc};
        use crate::schema::user_settings::{self, dsl as dsl_us};

        // let like_clause = format!("%{}%", location).to_lowercase();

        let mut filterer: Box<dyn BoxableExpression<user_connect::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl_uc::enable_push_notif.eq(true));

        if location != "*" && location != "" {
            match loc_kind {
                LocKind::Country => {
                    filterer =
                        Box::new(filterer.and(
                            lower(dsl_uc::latest_loc_full).like(format!("{}/%", location.to_lowercase())),
                        ));
                }
                LocKind::Province => {
                    filterer = Box::new(filterer.and(
                        lower(dsl_uc::latest_loc_full).like(format!("%/{}/%", location.to_lowercase())),
                    ));
                }
                LocKind::City => {
                    filterer =
                        Box::new(filterer.and(
                            lower(dsl_uc::latest_loc_full).like(format!("%/{}%", location.to_lowercase())),
                        ));
                }
                _ => (),
            }
        }

        user_connect::table
            .filter(filterer)
            .select(dsl_uc::app_id)
            .get_results::<String>(conn)
            .map_err(From::from)
    }

    /// FCM send push notification.
    pub fn push<'a>(
        &self,
        provider: &'a str,
        payload: &'a FCMPayloadData,
        conn: &'a PgConnection,
    ) -> Result<()> {
        if !self.server_key.is_empty() {
            // if let Ok(app_id) = self.get_user_app_id(payload.receiver_loc, conn) {
            if let Ok(app_ids) = self.get_user_app_ids(conn, payload.receiver_loc, payload.receiver_loc_kind)
            {
                if app_ids.len() == 0 {
                    debug!("No target to send notification");
                    return Ok(());
                }

                debug!("Sending to app_ids: {:?}", &app_ids);
                // let mut m_builder = MessageBuilder::new(&self.server_key, &app_id);
                let mut m_builder = MessageBuilder::new_multi(&self.server_key, &app_ids);

                if provider != "web" {
                    let mut n_builder = NotificationBuilder::new();
                    n_builder.title(payload.title);
                    n_builder.body(payload.message);
                    n_builder.sound("default");
                    n_builder.click_action(payload.click_action);

                    m_builder.notification(n_builder.finalize());
                }

                m_builder.priority(Priority::High);
                m_builder
                    .data(&FCMPayloadDataWire {
                        receiver_loc: payload.receiver_loc,
                        target_id: payload.target_id,
                        item: payload.item,
                        kind: payload.kind as i32,
                        title: payload.title,
                        message: payload.message,
                        created: payload.created,
                        click_action: payload.click_action,
                    })
                    .expect("Cannot set payload");

                let sending = self.client.send(m_builder.finalize());

                let mut core = Core::new().expect("cannot get new Core");

                let rv = core
                    .run(lazy(move || sending))
                    .expect("Cannot run reactor for sending push notif");

                debug!("Send push notification: {:?}", rv);
            }
        } else {
            debug!("FCM server key not set");
        }

        Ok(())
    }
}
