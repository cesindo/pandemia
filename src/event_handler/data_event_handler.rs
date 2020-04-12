//! Event handler for data records
use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    api::types,
    dao::{FeedDao, MapMarkerDao, NotifDao},
    event_handler::FCM,
    eventstream::{self, Event::*},
    geolocator,
    models::{Record, User},
    // notif_sender::send_notif,
    push_notif_handler::{FCMHandler, FCMPayloadData},
    result::Result,
    token,
    types::{FeedKind, MapMarkerKind, NotifKind},
    util,
    ID,
};

/// Event handler when new record updates found
pub fn new_record_update(
    old_record: &Option<Record>,
    new_record: &Record,
    conn: &PgConnection,
) -> Result<()> {
    if let Err(e) = update_feed_and_push_notif(old_record, new_record, conn) {
        error!("Cannot update feed or process push notif. {}", e);
    }

    // update map marker
    if let Err(e) = update_map_marker(old_record, new_record, conn) {
        error!("Cannot update map marker. {}", e);
    }

    Ok(())
}

fn update_map_marker(old_record: &Option<Record>, new_record: &Record, conn: &PgConnection) -> Result<()> {
    let dao = MapMarkerDao::new(conn);

    let scope_meta = new_record
        .meta
        .iter()
        .filter(|a| a.starts_with("loc_scope:"))
        .map(|a| a.as_str())
        .collect();

    if let Ok(Some(marker)) = dao.get_by_name(&new_record.loc, scope_meta) {
        // update
        let mut meta = marker.meta.clone();
        meta = meta
            .into_iter()
            .filter(|a| !a.starts_with("pandemic.total_"))
            .collect();
        meta.push(format!("pandemic.total_cases:{}", new_record.total_cases));
        meta.push(format!("pandemic.total_deaths:{}", new_record.total_deaths));
        meta.push(format!("pandemic.total_recovered:{}", new_record.total_recovered));
        dao.update_meta(marker.id, meta)?;
    } else {
        let loc_scope = meta_value_str!(new_record, "loc_scope");
        let latlong = geolocator::loc_to_ll(&format!("{} {}", new_record.loc, loc_scope), conn)?;
        let mut meta = new_record.meta.clone();
        meta.push(format!("pandemic.total_cases:{}", new_record.total_cases));
        meta.push(format!("pandemic.total_deaths:{}", new_record.total_deaths));
        meta.push(format!("pandemic.total_recovered:{}", new_record.total_recovered));
        dao.create(
            &new_record.loc,
            &format!("Info pandemi untuk area {}", new_record.loc),
            latlong.latitude,
            latlong.longitude,
            MapMarkerKind::PandemicInfo,
            &meta.iter().map(|a| a.as_str()).collect(),
        )?;
    }

    Ok(())
}

fn update_feed_and_push_notif(
    old_record: &Option<Record>,
    new_record: &Record,
    conn: &PgConnection,
) -> Result<()> {
    let feed_dao = FeedDao::new(conn);
    if let Some(old_record) = old_record {
        let diff = new_record.diff(old_record);
        debug!("diff: {:?}", diff);
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
                    receiver_loc: &new_record.loc,
                    receiver_loc_kind: new_record.loc_kind.into(),
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
                    receiver_loc: &new_record.loc,
                    receiver_loc_kind: new_record.loc_kind.into(),
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
                "+{} sembuh, {} total telah sembuh",
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
                    receiver_loc: &new_record.loc,
                    receiver_loc_kind: new_record.loc_kind.into(),
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
        if new_record.total_cases != 0 && new_record.total_deaths != 0 && new_record.total_recovered != 0 {
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
    }

    Ok(())
}
