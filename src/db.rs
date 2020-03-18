use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use std::env;

pub type DbConnMan = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref DB_CONN_POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
        let conn_man = ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL").expect("no DATABASE_URL env var"),
        );
        r2d2::Pool::builder()
            .max_size(2)
            .build(conn_man)
            .expect("Cannot build DB connection poll")
    };
}

pub fn connect(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).unwrap_or_else(|_| panic!("Cannot connect to `{}`", db_url))
}

pub fn clone() -> DbConnMan {
    DB_CONN_POOL.clone()
}
