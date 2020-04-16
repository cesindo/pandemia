//! Key value store mechanism
//!

use diesel::prelude::*;

use crate::{error::Error, result::Result, schema::kv_store, ID};

#[derive(Insertable, AsChangeset)]
#[table_name = "kv_store"]
struct NewKvStore<'a> {
    pub a_key: &'a str,
    pub a_val: &'a str,
}

/// Data Access Object for KvStore
pub struct KvStore<'a> {
    db: &'a PgConnection,
}

impl<'a> KvStore<'a> {
    #[doc(hidden)]
    pub fn new(conn: &'a PgConnection) -> Self {
        Self { db: conn }
    }
    /// Create new KvStore
    pub fn set(&self, a_key: &'a str, a_val: &'a str) -> Result<()> {
        use crate::schema::kv_store::{self, dsl};

        let entry = NewKvStore { a_key, a_val };
        diesel::insert_into(kv_store::table)
            .values(&entry)
            .on_conflict(dsl::a_key)
            .do_update()
            .set(&entry)
            .execute(self.db)?;
        // .map_err(From::from)?;

        Ok(())
    }

    /// Mendapatkan value dari kv store.
    pub fn get(&self, a_key: &str) -> Result<Option<String>> {
        use crate::schema::kv_store::{self, dsl};
        dsl::kv_store
            .filter(dsl::a_key.eq(a_key))
            .select(dsl::a_val)
            .first(self.db)
            .map_err(From::from)
            .map(|a| Some(a))
    }

    /// Delete entry
    pub fn delete(&self, a_key: &str) -> Result<()> {
        use crate::schema::kv_store::{self, dsl};

        diesel::delete(dsl::kv_store.filter(dsl::a_key.eq(a_key))).execute(self.db)?;
        Ok(())
    }

    /// Delete all values with specific criteria
    pub fn delete_by_values(&self, a_val: &str) -> Result<()> {
        use crate::schema::kv_store::{self, dsl};

        diesel::delete(dsl::kv_store.filter(dsl::a_val.eq(a_val))).execute(self.db)?;

        Ok(())
    }
}
