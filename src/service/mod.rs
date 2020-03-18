//! Modular & extendable Service interface

mod services;
mod auth;
mod user;
mod system;
mod pandemia;

pub use self::services::load_services;
pub use self::{
    auth::AuthService,
    user::UserService,
    system::SystemService,
    pandemia::PandemiaService
};

use crate::api;
use crate::api::*;
use actix_web::{http::Method, App, AsyncResponder, Error, Path, Result};

///! Base service interface
pub trait Service {
    /// Returns service name
    /// service name must unique between each other.
    fn name(&self) -> &'static str;

    /// Method untuk wiring API.
    fn wire_api(&self, builder: &mut ServiceApiBuilder);
}
