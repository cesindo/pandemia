//! Module untuk mengurus otorisasi
//!
//!

use chrono::{Duration, NaiveDateTime};
use diesel::{pg::PgConnection, prelude::*};

use crate::{
    error::{Error as PandemiaError, ErrorCode},
    models::AdminAccessToken,
    models::{AccessToken, User},
    prelude::*,
    schema::{access_tokens, admin_access_tokens},
    token,
    types::AccountKind,
    util, ID,
};

#[doc(hidden)]
#[derive(Insertable)]
#[table_name = "access_tokens"]
pub struct NewAccessToken<'a> {
    pub token: &'a str,
    pub user_id: ID,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Insertable)]
#[table_name = "admin_access_tokens"]
pub struct NewAdminAccessToken<'a> {
    pub token: &'a str,
    pub admin_id: ID,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

/// Untuk mengoperasikan skema data di database
pub struct AuthDao<'a> {
    db: &'a PgConnection,
}

impl<'a> AuthDao<'a> {
    /// Create new schema instance.
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Mendapatkan akun dari akses token.
    pub fn get_user_from_access_token(&self, access_token: &str) -> Result<User> {
        use crate::schema::users::dsl::users;

        // @TODO(robin): ini masih bisa diimprove dengan hanya menggunakan sekali call ke DB
        self.get_access_token(access_token)
            .map(|token| users.find(token.user_id).first(self.db).map_err(From::from))?
    }

    /// Mendapatkan akses token object dari string token.
    pub fn get_access_token(&self, access_token: &str) -> Result<AccessToken> {
        use crate::schema::access_tokens::dsl::access_tokens;

        access_tokens
            .find(access_token)
            .first(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan akses token object dari string token untuk Admin.
    pub fn get_admin_access_token(&self, access_token: &str) -> Result<AdminAccessToken> {
        use crate::schema::admin_access_tokens::dsl::admin_access_tokens;

        admin_access_tokens
            .find(access_token)
            .first(self.db)
            .map_err(From::from)
    }

    /// Generate access token, this write access token into database.
    pub fn generate_access_token(&self, user_id: ID) -> Result<AccessToken> {
        use crate::schema::access_tokens::{self, dsl};

        let now = chrono::Utc::now().naive_utc();

        // hapus token lama kalau ada
        diesel::delete(dsl::access_tokens.filter(dsl::user_id.eq(user_id))).execute(self.db)?;

        let token = NewAccessToken {
            token: &token::generate_access_token(),
            user_id,
            created: now,
            valid_thru: now
                .checked_add_signed(Duration::days(7))
                .expect("cannot assign valid_thru time"),
        };

        diesel::insert_into(access_tokens::table)
            .values(&token)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Generate admin access token
    pub fn generate_admin_access_token(&self, admin_id: ID) -> Result<AccessToken> {
        use crate::schema::admin_access_tokens::{self, dsl};

        let now = chrono::Utc::now().naive_utc();

        // hapus token lama kalau ada
        diesel::delete(dsl::admin_access_tokens.filter(dsl::admin_id.eq(admin_id))).execute(self.db)?;

        let token = NewAdminAccessToken {
            token: &token::generate_access_token(),
            admin_id,
            created: now,
            valid_thru: now
                .checked_add_signed(Duration::days(7))
                .expect("cannot assign valid_thru time"),
        };

        diesel::insert_into(admin_access_tokens::table)
            .values(&token)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan passhash
    pub fn get_passhash(&self, kind: AccountKind, id: ID) -> Result<String> {
        match kind {
            AccountKind::User => {
                use crate::schema::user_passhash::dsl;
                dsl::user_passhash
                    .filter(dsl::user_id.eq(id).and(dsl::deprecated.eq(false)))
                    .select(dsl::passhash)
                    .get_result::<String>(self.db)
                    .map_err(From::from)
            }
            AccountKind::Admin => {
                use crate::schema::admin_passhash::dsl;
                dsl::admin_passhash
                    .filter(dsl::admin_id.eq(id).and(dsl::deprecated.eq(false)))
                    .select(dsl::passhash)
                    .get_result::<String>(self.db)
                    .map_err(From::from)
            }
            // _ => Err(PandemiaError::BadRequest(
            //     ErrorCode::InvalidParameter as i32,
            //     "Kind not found".to_string(),
            // ))?,
        }
    }

    /// Periksa apakah akun terhubung dengan spesifik passhash.
    /// Mengembalikan true apabila valid (ada).
    pub fn valid_passhash(&self, user_id: ID, passhash: &str) -> bool {
        use crate::schema::user_passhash::dsl;

        dsl::user_passhash
            .filter(dsl::user_id.eq(user_id).and(dsl::passhash.eq(passhash)))
            .select(dsl::user_id)
            .get_result::<i64>(self.db)
            .is_ok()
    }

    /// Remove access token from db
    pub fn remove_access_token(&self, access_token: &str) -> Result<()> {
        use crate::schema::access_tokens::{self, dsl};
        diesel::delete(dsl::access_tokens.filter(dsl::token.eq(access_token))).execute(self.db)?;
        Ok(())
    }

    /// Clear user's access tokens by user id
    pub fn clear_access_token_by_user_id(&self, user_id: ID) -> Result<()> {
        use crate::schema::access_tokens::{self, dsl};
        diesel::delete(dsl::access_tokens.filter(dsl::user_id.eq(user_id))).execute(self.db)?;
        Ok(())
    }

    /// Clear admin's access tokens by admin id
    pub fn clear_access_token_by_admin_id(&self, admin_id: ID) -> Result<()> {
        use crate::schema::admin_access_tokens::{self, dsl};
        diesel::delete(dsl::admin_access_tokens.filter(dsl::admin_id.eq(admin_id))).execute(self.db)?;
        Ok(())
    }
}
