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
    types::FeedKind,
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

        if diff.new_cases > 0 {
            if let Err(e) = feed_dao.create(
                0,
                "",
                &new_record.loc,
                FeedKind::NewCases,
                &format!("+{} kasus baru, total {}", diff.new_cases, new_record.total_cases),
                &vec![],
                &vec![],
            ) {
                error!("cannot create feed. {}", e);
            }
        }

        if diff.new_deaths > 0 {
            if let Err(e) = feed_dao.create(
                0,
                "",
                &new_record.loc,
                FeedKind::NewDeaths,
                &format!(
                    "+{} meninggal dunia, total {} yang telah meninggal",
                    diff.new_deaths, new_record.total_deaths
                ),
                &vec![],
                &vec![],
            ) {
                error!("cannot create feed. {}", e);
            }
        }

        if diff.new_recovered > 0 {
            if let Err(e) = feed_dao.create(
                0,
                "",
                &new_record.loc,
                FeedKind::NewRecovered,
                &format!(
                    "+{} sembuh, total {} yang telah sembuh",
                    diff.new_recovered, new_record.total_recovered
                ),
                &vec![],
                &vec![],
            ) {
                error!("cannot create feed. {}", e);
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

    // // Send push notification
    // FCM.push(
    //     "fcm",
    //     &FCMPayloadData {
    //         receiver_id: task.assignee_id,
    //         target_id: task.id,
    //         kind: NotifKind::GotNewTask,
    //         title: "New Task",
    //         item: &serde_json::to_string::<types::Task>(&task.into())?,
    //         message: &format!("[{}] {}", project_name, task.name),
    //         created: crate::util::now(),
    //         click_action: "FLUTTER_NOTIFICATION_CLICK",
    //     },
    //     conn,
    // )?;

    Ok(())
}
