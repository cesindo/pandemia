//! Event handler for data records
use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    api::types,
    dao::{FeedDao, NotifDao},
    event_handler::FCM,
    eventstream::{self, Event::*},
    models::{Record, User},
    notif_sender::send_notif,
    push_notif_handler::{FCMHandler, FCMPayloadData},
    result::Result,
    token,
    types::{FeedKind, NotifKind},
    util, ID,
};

/// Event handler when new record updates found
pub fn new_record_update(
    old_record: &Option<Record>,
    new_record: &Record,
    conn: &PgConnection,
) -> Result<()> {
    let feed_dao = FeedDao::new(conn);
    if let Some(old_record) = old_record {
        let diff = new_record.diff(old_record);
        let title = if new_record.loc != "Indonesia" {
            format!("Update Wilayah {}", new_record.loc)
        } else {
            "Update Nasional".to_string()
        };

        if diff.new_cases > 0 {
            let message = format!("+{} kasus baru, total {}", diff.new_cases, new_record.total_cases);

            if let Err(e) = feed_dao.create(
                0,
                "",
                &new_record.loc,
                FeedKind::NewCases,
                &message,
                &vec![],
                &vec![],
            ) {
                error!("cannot create feed. {}", e);
            }

            // Send push notification
            if let Err(e) = FCM.push(
                "fcm",
                &FCMPayloadData {
                    receiver_id: 0,
                    target_id: 0,
                    kind: NotifKind::NewCases,
                    title: &title,
                    item: "",
                    message: &message,
                    created: util::now(),
                    click_action: "FLUTTER_NOTIFICATION_CLICK",
                },
                conn,
            ) {
                error!("cannot send push notif. {}", e);
            }
        }

        if diff.new_deaths > 0 {
            let message = format!(
                "+{} meninggal dunia, total {} yang telah meninggal",
                diff.new_deaths, new_record.total_deaths
            );
            if let Err(e) = feed_dao.create(
                0,
                "",
                &new_record.loc,
                FeedKind::NewDeaths,
                &message,
                &vec![],
                &vec![],
            ) {
                error!("cannot create feed. {}", e);
            }

            // Send push notification
            if let Err(e) = FCM.push(
                "fcm",
                &FCMPayloadData {
                    receiver_id: 0,
                    target_id: 0,
                    kind: NotifKind::NewDeaths,
                    title: &title,
                    item: "",
                    message: &message,
                    created: util::now(),
                    click_action: "FLUTTER_NOTIFICATION_CLICK",
                },
                conn,
            ) {
                error!("cannot send push notif. {}", e);
            }
        }

        if diff.new_recovered > 0 {
            let message = format!(
                "+{} sembuh, total {} orang telah sembuh",
                diff.new_recovered, new_record.total_recovered
            );
            if let Err(e) = feed_dao.create(
                0,
                "",
                &new_record.loc,
                FeedKind::NewRecovered,
                &message,
                &vec![],
                &vec![],
            ) {
                error!("cannot create feed. {}", e);
            }

            // Send push notification
            if let Err(e) = FCM.push(
                "fcm",
                &FCMPayloadData {
                    receiver_id: 0,
                    target_id: 0,
                    kind: NotifKind::NewRecovered,
                    title: &title,
                    item: "",
                    message: &message,
                    created: util::now(),
                    click_action: "FLUTTER_NOTIFICATION_CLICK",
                },
                conn,
            ) {
                error!("cannot send push notif. {}", e);
            }
        }
    } else {
        if let Err(e) = feed_dao.create(
            0,
            "",
            &new_record.loc,
            FeedKind::Info,
            &format!(
                "{} positif, {} meninggal, {} sembuh",
                new_record.total_cases, new_record.total_deaths, new_record.total_recovered
            ),
            &vec![],
            &vec![],
        ) {
            error!("cannot create feed. {}", e);
        }
    }

    Ok(())
}
