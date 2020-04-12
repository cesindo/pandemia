//! Dao implementation for Admin
//!

use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    error::{Error as PdmError, ErrorCode},
    models::{Admin, ResetPasswordAdmin},
    result::Result,
    schema::{admin_passhash, admins, reset_password_admins},
    token, util, ID,
};

#[derive(Insertable)]
#[table_name = "admins"]
struct NewAdmin<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub labels: &'a Vec<String>,
}

#[doc(hidden)]
#[derive(Insertable, AsChangeset)]
#[table_name = "reset_password_admins"]
pub struct NewResetPasswordKey<'a> {
    pub admin_id: ID,
    pub token: &'a str,
    pub expiration: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "admin_passhash"]
#[doc(hidden)]
pub struct NewAdminPasshash<'a> {
    pub admin_id: ID,
    pub passhash: &'a str,
    pub deprecated: bool,
    pub ver: i32,
}

/// Data Access Object for Admin
#[derive(Dao)]
#[table_name = "admins"]
pub struct AdminDao<'a> {
    db: &'a PgConnection,
}

impl<'a> AdminDao<'a> {
    /// Create new Admin
    pub fn create(
        &self,
        name: &'a str,
        email: &'a str,
        phone_num: &'a str,
        password: &'a str,
        labels: &'a Vec<String>,
    ) -> Result<Admin> {
        self.db.build_transaction().read_write().run::<_, _, _>(|| {
            let admin: Admin = diesel::insert_into(admins::table)
                .values(&NewAdmin {
                    name,
                    email,
                    phone_num,
                    labels,
                })
                .get_result(self.db)?;

            // tambahkan password baru
            let passhash = &crate::crypto::get_passhash(password);
            diesel::insert_into(admin_passhash::table)
                .values(&NewAdminPasshash {
                    admin_id: admin.id,
                    passhash,
                    deprecated: false,
                    ver: 1,
                })
                .execute(self.db)?;

            Ok(admin)
        })
    }

    /// Mendapatkan admin berdasarkan emailnya.
    pub fn get_by_email(&self, email: &str) -> Result<Admin> {
        use crate::schema::admins::dsl;
        dsl::admins
            .filter(dsl::email.eq(email))
            .first(self.db)
            .map_err(From::from)
    }

    /// Request token untuk reset password.
    pub fn reset_password(&self, admin_id: ID, name: String, email: String) -> Result<()> {
        use crate::schema::reset_password_admins::dsl;

        let expiration = Some(util::now() + chrono::Duration::days(1));
        let token = &token::generate_token();
        let new_entry = NewResetPasswordKey {
            admin_id,
            token,
            expiration,
        };

        debug!("reset password token: {}", token);

        diesel::insert_into(reset_password_admins::table)
            .values(&new_entry)
            .on_conflict(dsl::admin_id)
            .do_update()
            .set(&new_entry)
            .execute(self.db)?;

        Ok(())
    }

    /// Verifikasi token untuk reset password.
    pub fn verify_reset_password(&self, admin_id: ID, token: &str) -> Result<()> {
        use crate::schema::reset_password_admins::dsl;

        let reset_password: ResetPasswordAdmin = dsl::reset_password_admins.find(admin_id).first(self.db)?;
        let now = util::now().timestamp();

        match (reset_password.expiration, reset_password.token) {
            (Some(expiration), old_token) => {
                let exp = expiration.timestamp();
                if exp < now {
                    Err(PdmError::BadRequest(
                        ErrorCode::InvalidParameter as i32,
                        "Token has expired".to_string(),
                    ))?
                } else if old_token != token {
                    Err(PdmError::InvalidParameter("Mismatch token".to_string()))
                } else {
                    Ok(())
                }
            }
            _ => Err(PdmError::NotFound("Token not found".to_string()))?,
        }
    }

    /// Remove reset password token.
    pub fn remove_reset_password(&self, admin_id: ID) -> Result<()> {
        use crate::schema::reset_password_admins::dsl;

        diesel::delete(dsl::reset_password_admins.find(admin_id)).execute(self.db)?;

        Ok(())
    }

    /// Setting admin's password
    pub fn set_password(&self, admin_id: ID, password: &str) -> Result<()> {
        use crate::schema::admin_passhash::dsl;

        let _ = self.get_by_id(admin_id)?;

        self.db.build_transaction().read_write().run(|| {
            let passhash = &crate::crypto::get_passhash(password);

            // dipresiasi password lama
            diesel::update(
                dsl::admin_passhash.filter(dsl::admin_id.eq(admin_id).and(dsl::deprecated.eq(false))),
            )
            .set(dsl::deprecated.eq(true))
            .execute(self.db)?;

            // tambahkan password baru
            diesel::insert_into(admin_passhash::table)
                .values(&NewAdminPasshash {
                    admin_id,
                    passhash,
                    deprecated: false,
                    ver: 1,
                })
                .execute(self.db)?;

            Ok(())
        })
    }
}
