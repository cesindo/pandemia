//! Dao implementation for Notif
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{models::Notif, result::Result, schema::notifs, types::NotifKind, ID};

#[derive(Insertable)]
#[table_name = "notifs"]
struct NewNotif<'a> {
    pub kind: i16,
    pub text: &'a str,
    pub initiator_id: ID,
    pub receiver_id: ID,
    pub keywords: &'a [&'a str],
    pub meta: &'a [&'a str],
}

/// Data Access Object for Notif
#[derive(Dao)]
#[table_name = "notifs"]
pub struct NotifDao<'a> {
    db: &'a PgConnection,
}

impl<'a> NotifDao<'a> {
    /// Create new Notif
    pub fn create(
        &self,
        kind: NotifKind,
        text: &'a str,
        initiator_id: ID,
        receiver_id: ID,
        keywords: &'a [&'a str],
        meta: &'a [&'a str],
    ) -> Result<Notif> {
        use crate::schema::notifs::{self, dsl};

        diesel::insert_into(notifs::table)
            .values(&NewNotif {
                kind: kind as i16,
                text,
                initiator_id,
                receiver_id,
                keywords,
                meta,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mark read or unread notif
    pub fn set_notif_read(&self, id: ID, state: bool) -> Result<()> {
        use crate::schema::notifs::{self, dsl};

        diesel::update(dsl::notifs.filter(dsl::id.eq(id)))
            .set(dsl::read.eq(state))
            .execute(self.db)?;

        Ok(())
    }

    /// Mark all as read or unread notif
    pub fn mark_all_read(&self, receiver_id: ID, state: bool) -> Result<()> {
        use crate::schema::notifs::{self, dsl};

        diesel::update(dsl::notifs.filter(dsl::receiver_id.eq(receiver_id)))
            .set(dsl::read.eq(state))
            .execute(self.db)?;

        Ok(())
    }

    /// Get list of ReturnType
    pub fn get_notifs_by_receiver_id(
        &self,
        receiver_id: ID,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<Notif>, i64)> {
        use crate::schema::notifs::{self, dsl};

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        Ok((
            dsl::notifs
                .filter(dsl::receiver_id.eq(receiver_id))
                .offset(offset)
                .limit(limit)
                .order(dsl::id.desc())
                .load(self.db)?,
            dsl::notifs
                .filter(dsl::receiver_id.eq(receiver_id))
                .count()
                .get_result(self.db)?,
        ))
    }
}
