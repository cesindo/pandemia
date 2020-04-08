use diesel::{connection::Connection, pg::PgConnection};
use pandemia::api::user::types;
use pandemia::api::ApiResult;
use pandemia::auth;
use pandemia::crypto::*;
use pandemia::models;
use pandemia::prelude::*;
use pandemia::user_dao::*;
use pandemia::{api::types::IdQuery, util};
use serde_json::Value as JsonValue;

use crate::{ApiKind, TestKit, TestKitApi, ID};

use std::{
    env,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct UserWithKey {
    pub user: types::User,
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl UserWithKey {
    pub fn new(user: types::User, public_key: PublicKey, secret_key: SecretKey) -> Self {
        Self {
            user,
            public_key,
            secret_key,
        }
    }
}

#[allow(dead_code)]
pub struct TestHelper {
    testkit: TestKit,
}

impl TestHelper {
    pub fn new(testkit: &TestKit) -> Self {
        Self {
            testkit: testkit.clone(),
        }
    }

    fn get_db<'a>() -> MutexGuard<'a, PgConnection> {
        lazy_static! {
            static ref PG_CONN_FOR_TEST: Arc<Mutex<PgConnection>> = Arc::new(Mutex::new(
                PgConnection::establish(
                    &env::var("DATABASE_TEST_URL").expect("No DATABASE_TEST_URL env var")
                )
                .expect("Cannot connect to db")
            ));
        }

        PG_CONN_FOR_TEST.lock().unwrap()
    }

    pub fn get_user_by_id(&self, id: ID) -> Result<models::User> {
        let db = Self::get_db();
        let dao = UserDao::new(&db);
        dao.get_by_id(id)
    }

    /// Menggenerasikan akses token langsung dari database,
    /// Tidak melalui API endpoint `/authorize`.
    pub fn gen_access_token_for(&self, id: ID) -> Result<models::AccessToken> {
        let db = Self::get_db();
        let dao = auth::AuthDao::new(&db);
        dao.generate_access_token(id).map_err(From::from)
    }

    pub fn cleanup_registered_user(&self, token: &str) {
        let db = Self::get_db();
        let dao = UserDao::new(&db);
        let _ = dao.cleanup_registered_user(token);
    }

    pub fn generate_full_name(&self) -> String {
        // @TODO(robin): mungkin nantinya gunakan tool seperti ini: https://github.com/fnichol/names ?
        util::random_string(10)
    }

    pub fn generate_amount(&self) -> f64 {
        util::random_number_f64()
    }

    pub fn generate_email(&self) -> String {
        format!("{}@{}.com", util::random_string(10), util::random_string(5)).to_lowercase()
    }

    pub fn generate_phone_num(&self) -> String {
        let nums: String = (0..10).map(|_| util::random_number().to_string()).collect();
        format!("+628{}", nums)
    }

    /// Menggenerasikan beberapa akun sekaligus,
    /// ini tidak via rest API, tapi langsung ke database.
    pub fn generate_users(&self, count: usize) -> Vec<UserWithKey> {
        let db = Self::get_db();
        let schema = UserDao::new(&db);
        let mut rv = vec![];
        for _ in 0..count {
            let new_user = NewUser {
                full_name: &self.generate_full_name(),
                email: &self.generate_email(),
                phone_num: &self.generate_phone_num(),
                active: true,
                register_time: util::now(),
            };
            let (user, (public_key, secret_key)) =
                schema.create_user(&new_user, None).expect("cannot create user");
            rv.push(UserWithKey::new(user.into(), public_key, secret_key));
        }
        rv
    }

    /// Menghapus akun berdasarkan ID.
    pub fn cleanup_user_by_id(&self, user_id: ID) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        let _ = schema.delete_user_by_id(user_id);
    }

    /// Menghapus akun
    pub fn cleanup_user(&self, user: types::User) {
        self.cleanup_user_by_id(user.id);
    }

    /// Bersihkan data akun berdasarkan list dari ID-nya.
    pub fn cleanup_users(&self, user_ids: Vec<ID>) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        schema.cleanup_users(user_ids);
    }
}

pub struct ApiHelper<'a> {
    testkit: &'a TestKit,
}

impl<'a> ApiHelper<'a> {
    pub fn new(testkit: &'a TestKit) -> Self {
        Self { testkit }
    }

    // /// Register user
    // /// Mengembalikan token untuk aktivasi.
    // pub fn register_user(&self, user_name: &str, email: &str, phone_number: &str) -> ApiResult<String> {
    //     let api = self.testkit.api();

    //     let data = RegisterUser {
    //         full_name: user_name.to_owned(),
    //         email: email.to_owned(),
    //         phone_num: phone_number.to_owned(),
    //     };

    //     api.public(ApiKind::User)
    //         .query(&data)
    //         .post("v1/user/register")
    //         .expect("create user")
    // }

    // /// Aktivasi akun menggunakan token yang telah didapat dari hasil register.
    // pub fn activate_user(&self, token: String, password: &str) -> ApiResult<types::User> {
    //     let api = self.testkit.api();

    //     let data = ActivateUser {
    //         token,
    //         password: password.to_owned(),
    //     };

    //     api.public(ApiKind::User)
    //         .query(&data)
    //         .post::<ApiResult<types::User>>("v1/user/activate")
    //         .expect("activate user")
    // }
}
