//! UserDao operation a.k.a DAO, digunakan untuk melakukan operasi seperti
//! membuat akun baru, update, dan delete.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::{
    crypto::{self, PublicKey, SecretKey},
    error::Error as PandemiaError,
    models::*,
    result::Result,
    schema::*,
    sqlutil::lower,
    token, ID,
};

use std::sync::Arc;

#[derive(Insertable)]
#[table_name = "register_users"]
#[doc(hidden)]
pub struct NewRegisterUser<'a> {
    pub token: &'a str,
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub register_time: NaiveDateTime,
    pub code: &'a str,
}

#[derive(Insertable)]
#[table_name = "users"]
#[doc(hidden)]
pub struct NewUser<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "user_passhash"]
#[doc(hidden)]
pub struct NewUserPasshash<'a> {
    pub user_id: ID,
    pub passhash: &'a str,
    pub deprecated: bool,
    pub ver: i32,
}

#[derive(Insertable)]
#[table_name = "user_keys"]
#[doc(hidden)]
pub struct NewUserKey {
    pub user_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub active: bool,
}

#[doc(hidden)]
#[derive(Insertable, AsChangeset)]
#[table_name = "user_connect"]
pub struct NewUserConnect<'a> {
    pub user_id: ID,
    pub device_id: &'a str,
    pub provider_name: &'a str,
    pub app_id: &'a str,
    pub latest_loc: &'a str,
    pub latest_loc_full: &'a str,
    pub latest_loc_long: f64,
    pub latest_loc_lat: f64,
}

/// Untuk mengoperasikan skema data di database
#[derive(Dao)]
pub struct UserDao<'a> {
    db: &'a PgConnection,
}

impl<'a> UserDao<'a> {
    /// Mendapatkan akun berdasarkan emailnya.
    pub fn get_by_email(&self, email: &str) -> Result<User> {
        use crate::schema::users::dsl;
        dsl::users
            .filter(dsl::email.eq(email))
            .first(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan akun berdasarkan nomor telp-nya.
    pub fn get_by_phone_num(&self, phone: &str) -> Result<User> {
        use crate::schema::users::dsl;
        dsl::users
            .filter(dsl::phone_num.eq(phone))
            .first(self.db)
            .map_err(From::from)
    }

    /// Setting user's password
    pub fn set_password(&self, user_id: ID, password: &str) -> Result<()> {
        use crate::schema::user_passhash::dsl;

        let _ = self.get_by_id(user_id)?;

        self.db.build_transaction().read_write().run(|| {
            let passhash = &crate::crypto::get_passhash(password);

            // dipresiasi password lama
            diesel::update(
                dsl::user_passhash.filter(dsl::user_id.eq(user_id).and(dsl::deprecated.eq(false))),
            )
            .set(dsl::deprecated.eq(true))
            .execute(self.db)?;
            // .map_err(From::from)?;

            // tambahkan password baru
            diesel::insert_into(user_passhash::table)
                .values(&NewUserPasshash {
                    user_id,
                    passhash,
                    deprecated: false,
                    ver: 1,
                })
                .execute(self.db)?;
            // .map_err(From::from)?;

            Ok(())
        })
    }

    /// Mendaftarkan akun baru.
    /// Mengembalikan ID dari registered user (bukan [User]: pandemia::models::User)
    /// karena user belum aktif, untuk mengaktifkannya perlu memanggil
    /// perintah [UserDao::activate_registered_user].
    pub fn register_user(&self, full_name: &str, email: &str, phone_num: &str) -> Result<String> {
        use crate::schema::{register_users::dsl as dsl_ra, users::dsl as dsl_user};

        if full_name == "" {
            Err(PandemiaError::InvalidParameter(
                "full name cannot be empty".to_string(),
            ))?
        }
        if email == "" {
            Err(PandemiaError::InvalidParameter(
                "email cannot be empty".to_string(),
            ))?
        }
        // @TODO(robin): lakukan validasi format nomor telp
        if phone_num == "" {
            Err(PandemiaError::InvalidParameter(
                "phone_num cannot be empty".to_string(),
            ))?
        }

        // tolak akun dengan nama-nama tertentu
        // @TODO(robin): buat konfigurable
        if full_name == "nobody" {
            warn!("Name exception to register: `{}`", full_name);
            Err(PandemiaError::Unauthorized)?
        }

        // apabila sudah exists di registered_users table
        // kembalikan token-nya aja
        if let Ok(ra) = dsl_ra::register_users
            .filter(dsl_ra::email.eq(email).or(dsl_ra::phone_num.eq(phone_num)))
            .first::<RegisterUser>(self.db)
        {
            return Ok(ra.token);
        }

        // check apakah akun dengan email/phone sama sudah ada
        let exists = dsl_user::users
            .filter(dsl_user::email.eq(email).or(dsl_user::phone_num.eq(phone_num)))
            .select(dsl_user::id)
            .first::<ID>(self.db)
            .is_ok();

        if exists {
            Err(PandemiaError::AlreadyExists)?
        }

        let new_reg_user = NewRegisterUser {
            token: &token::generate_token(),
            full_name,
            email,
            phone_num,
            register_time: Utc::now().naive_utc(),
            code: &token::generate_activation_code(),
        };

        diesel::insert_into(register_users::table)
            .values(&new_reg_user)
            .returning(dsl_ra::token)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mengaktifkan akun yang telah melakukan registrasi tapi belum aktif.
    pub fn activate_registered_user(&self, token: String) -> Result<User> {
        use crate::schema::user_keys::{self, dsl as ak_dsl};
        use crate::schema::{register_users, users};

        self.db.build_transaction().read_write().run(|| {
            let reg_acc: RegisterUser = register_users::dsl::register_users
                .find(token.clone())
                .first(self.db)?;
            // .map_err(From::from)?;

            let new_user = NewUser {
                full_name: &reg_acc.full_name,
                email: &reg_acc.email,
                phone_num: &reg_acc.phone_num,
                active: true,
                register_time: Utc::now().naive_utc(),
            };

            let user = diesel::insert_into(users::table)
                .values(&new_user)
                .get_result::<User>(self.db)?;
            // .map_err(From::from)?;

            // Buatkan key pair untuk akun yang baru saja dibuat.
            let (pub_key, secret_key) = crypto::gen_keypair();

            diesel::insert_into(user_keys::table)
                .values(&NewUserKey {
                    user_id: user.id,
                    pub_key: pub_key.to_hex(),
                    secret_key: secret_key.to_hex(),
                    active: true,
                })
                .execute(self.db)?;

            // delete reference in registered users table
            diesel::delete(register_users::dsl::register_users.find(token)).execute(self.db)?;

            Ok(user)
        })
    }

    /// Mendapatkan informasi key untuk akun.
    pub fn get_user_key(&self, user_id: ID) -> Result<UserKey> {
        use crate::schema::user_keys::{self, dsl as ak_dsl};
        use crate::schema::users;

        ak_dsl::user_keys
            .filter(ak_dsl::user_id.eq(user_id))
            .first(self.db)
            .map_err(From::from)
    }

    /// Buat akun baru secara langsung.
    pub fn create_user(
        &self,
        new_user: &NewUser,
        connect: Option<NewUserConnect>,
    ) -> Result<(User, (PublicKey, SecretKey))> {
        self.db.build_transaction().read_write().run(|| {
            let (user, keypair) = {
                use crate::schema::user_keys::{self, dsl as ak_dsl};
                use crate::schema::users;
                let user = diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result::<User>(self.db)?;

                // Buatkan key pair untuk akun yang baru saja dibuat.
                let keypair = crypto::gen_keypair();

                diesel::insert_into(user_keys::table)
                    .values(&NewUserKey {
                        user_id: user.id,
                        pub_key: keypair.0.to_hex(),
                        secret_key: keypair.1.to_hex(),
                        active: true,
                    })
                    .execute(self.db)?;

                (user, keypair)
            };

            if let Some(mut new_user_connect) = connect {
                use crate::schema::user_connect::{self, dsl};
                new_user_connect.user_id = user.id;
                diesel::insert_into(user_connect::table)
                    .values(&new_user_connect)
                    .on_conflict(dsl::device_id)
                    .do_update()
                    .set(&new_user_connect)
                    .execute(self.db)?;
            }

            Ok((user, keypair))
        })
    }

    /// Clean up registered user by token
    pub fn cleanup_registered_user(&self, token: &str) -> Result<usize> {
        use crate::schema::register_users::dsl;

        diesel::delete(dsl::register_users.filter(dsl::token.eq(token)))
            .execute(self.db)
            .map_err(From::from)
    }

    // /// Get multiple users
    // pub fn get_users(&self, offset: i64, limit: i64) -> Result<Vec<User>> {
    //     use crate::schema::users;
    //     use crate::schema::users::dsl;

    //     dsl::users
    //         .filter(dsl::id.ne(0))
    //         .offset(offset)
    //         .limit(limit)
    //         .load(self.db)
    //         .map_err(From::from)
    // }

    // /// Mendapatkan jumlah akun keseluruhan di dalam database.
    // pub fn get_user_count(&self) -> Result<i64> {
    //     use crate::schema::users;
    //     use crate::schema::users::dsl;

    //     dsl::users
    //         .select(diesel::dsl::count(dsl::id))
    //         .first(self.db)
    //         .map_err(From::from)
    // }

    /// Mencari akun berdasarkan kata kunci
    /// Ini mengembalikan tidak hanya daftar akun tetapi juga jumlah
    /// akun yang ada sesuai kata kunci tersebut.
    pub fn search(&self, keyword: &str, offset: i64, limit: i64) -> Result<(Vec<User>, i64)> {
        use crate::schema::users;
        use crate::schema::users::dsl;

        let like_clause = format!("%{}%", keyword).to_lowercase();

        let filterer = dsl::id.ne(0).and(
            lower(dsl::full_name)
                .like(&like_clause)
                .or(lower(dsl::email).like(&like_clause)),
        );

        let entries = dsl::users
            .filter(filterer)
            .offset(offset)
            .limit(limit)
            .load(self.db)?;

        let count = dsl::users
            .select(diesel::dsl::count(dsl::id))
            .filter(filterer)
            .first(self.db)?;

        Ok((entries, count))
    }

    /// Create user connect app id untuk spesifik user,
    /// digunakan untuk event push notif.
    pub fn create_user_connect(
        &self,
        user_id: ID,
        device_id: &str,
        provider_name: &str,
        app_id: &str,
        latest_loc: &str,
        latest_loc_full: &str,
    ) -> Result<()> {
        use crate::schema::user_connect::dsl;

        let user_connect = NewUserConnect {
            user_id,
            device_id,
            provider_name,
            app_id,
            latest_loc,
            latest_loc_full,
            latest_loc_long: 0.0,
            latest_loc_lat: 0.0,
        };

        diesel::insert_into(user_connect::table)
            .values(&user_connect)
            .on_conflict(dsl::device_id)
            .do_update()
            .set(&user_connect)
            .execute(self.db)?;

        Ok(())
    }

    /// Update user location by device_id
    pub fn update_user_location(
        &self,
        device_id: &str,
        latest_loc: &str,
        latest_loc_full: &str,
    ) -> Result<()> {
        use crate::schema::user_connect::{self, dsl};
        diesel::update(dsl::user_connect.filter(dsl::device_id.eq(device_id)))
            .set((
                dsl::latest_loc.eq(latest_loc),
                dsl::latest_loc_full.eq(latest_loc_full),
            ))
            .execute(self.db)?;
        Ok(())
    }

    /// Remove user connect app id untuk spesifik user.
    pub fn remove_user_connect(&self, device_id: &str, provider_name: &str, app_id: &str) -> Result<()> {
        use crate::schema::user_connect::dsl;

        diesel::delete(
            dsl::user_connect.filter(
                dsl::device_id
                    .eq(device_id)
                    .and(dsl::app_id.eq(app_id).and(dsl::provider_name.eq(provider_name))),
            ),
        )
        .execute(self.db)?;

        Ok(())
    }

    /// Remove user connect app id berdasarkan user id
    pub fn remove_user_connect_by_id(&self, device_id: &str) -> Result<()> {
        use crate::schema::user_connect::dsl;

        diesel::delete(dsl::user_connect.filter(dsl::device_id.eq(device_id))).execute(self.db)?;

        Ok(())
    }
}

/// UserDao untuk memudahkan integration testing
#[cfg(feature = "with-test")]
pub struct TestSchema<'a> {
    db: &'a PgConnection,
}

#[cfg(feature = "with-test")]
impl<'a> TestSchema<'a> {
    #[doc(hidden)]
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Menghapus akun secara batch
    pub fn cleanup_users(&self, user_ids: Vec<ID>) {
        use crate::schema::user_passhash::dsl as acp_dsl;
        use crate::schema::users;
        use crate::schema::users::dsl;

        let _ = self
            .db
            .build_transaction()
            .read_write()
            .run::<(), diesel::result::Error, _>(|| {
                for id in user_ids {
                    diesel::delete(acp_dsl::user_passhash.filter(acp_dsl::user_id.eq(id)))
                        .execute(self.db)?;
                    diesel::delete(dsl::users.filter(dsl::id.eq(id))).execute(self.db)?;
                }
                Ok(())
            });
    }

    /// Hapus akun berdasarkan id
    pub fn delete_user_by_id(&self, id: ID) -> Result<usize> {
        use crate::schema::users::dsl;
        diesel::delete(dsl::users.find(id))
            .execute(self.db)
            .map_err(From::from)
    }
}
