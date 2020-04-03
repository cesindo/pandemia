//! Notification sender, send notification to user
//!
// use diesel::prelude::*;

// use crate::{
//     api::types::{self, ToApiType},
//     dao::{NotifDao, UserDao},
//     event_handler::FCM,
//     models::{self, HasID, User},
//     push_notif_handler::{FCMHandler, FCMPayloadData},
//     result::Result,
//     token,
//     types::NotifKind,
//     util, ID,
// };

// /// Send notif to some account
// pub fn send_notif<Y, T>(
//     notif_kind: NotifKind,
//     initiator: &User,
//     receiver_loc: &str,
//     title: &str,
//     message: &str,
//     item: T,
//     only_push: bool,
//     conn: &PgConnection,
// ) -> Result<()>
// where
//     Y: serde::Serialize,
//     T: ToApiType<Y> + HasID + serde::Serialize,
// {
//     // if initiator.id == receiver_id {
//     //     // just ignore
//     //     return Ok(());
//     // }

//     // if !only_push {
//     //     // Sen app notif
//     //     if let Err(e) = NotifDao::new(conn).create(notif_kind, message, initiator.id, receiver_id, &[], &[]) {
//     //         error!("Cannot create notif mention for user {}. {}", receiver_id, e);
//     //     }
//     // }

//     // Send push notification
//     FCM.push(
//         "fcm",
//         &FCMPayloadData {
//             receiver_loc,
//             target_id: item.get_id(),
//             item: &serde_json::to_string::<Y>(&item.to_api_type(&conn))?,
//             kind: notif_kind,
//             title: &title,
//             message: &message,
//             created: util::now(),
//             click_action: "FLUTTER_NOTIFICATION_CLICK",
//         },
//         conn,
//     )?;

//     Ok(())
// }
