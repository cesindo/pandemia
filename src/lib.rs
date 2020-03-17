//! Pandemia
//!
//! # Fitur
//!
//! * Service & Http rest API management.
//! * Authentication & Authorization.
//! * Multi service implementation.
//!
#![deny(missing_docs)]
#![allow(unused_imports, unused_variables, dead_code, unused_macros)]
#![allow(clippy::new_without_default)]

// extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate r2d2;
#[macro_use]
extern crate diesel;
extern crate futures;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate env_logger;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate pandemia_proc_macro;

extern crate byteorder;
extern crate ed25519_dalek;
extern crate hex;
extern crate rand;
extern crate rsnowflake;
extern crate sha2;
#[macro_use]
extern crate lazy_static;
extern crate bcrypt;
#[macro_use]
extern crate validator_derive;
extern crate validator;

#[macro_use]
mod macros;
pub mod api;
pub mod auth;
pub mod crypto;
mod db;
pub mod error;
pub mod models;
mod result;
mod schema;
pub mod user_dao;
pub mod admin_dao;
pub mod dao;
pub mod service;
mod sqlutil;
pub mod token;
pub mod util;
mod valid;
pub mod web;

pub mod eventstream;

/// Type alias for ID in integer
pub type ID = i64;

/// Common use (prelude) exports.
#[doc(hidden)]
pub mod prelude {
    pub use super::{
        api::{
            self, ApiAccess, ApiAggregator, ApiBuilder, ApiServer, AppState, ServiceApiBuilder,
            ServiceApiConfig, ServiceApiScope,
        },
        result::Result,
        user_dao::UserDao,
        service::{UserService, Service},
        valid::{Expirable, Validable},
    };
}

