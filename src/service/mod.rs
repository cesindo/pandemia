//! Modular & extendable Service interface

mod auth;
mod pandemia;
mod services;
mod system;
mod user;

pub use self::services::load_services;
pub use self::{auth::AuthService, pandemia::PandemiaService, system::SystemService, user::UserService};

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
