//! Modular & extendable Service interface

mod services;
mod auth;
mod user;
mod system;

pub use self::services::load_services;
pub use self::{
    auth::AuthService,
    user::UserService,
    system::SystemService
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

/// Service contoh, kamu bisa mencontoh bagaimana caranya membuat service
/// dengan melihat kode [ExampleService] ini.
pub struct ExampleService;

impl Service for ExampleService {
    fn name(&self) -> &'static str {
        "pandemia"
    }
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}

/// Contoh API public untuk service contoh [[ExampleService]].
struct PublicApi {}

impl PublicApi {

    pub fn info_req(state: &AppState, query: (), req: &api::HttpRequest) -> api::Result<String> {
        Ok(concat!("version: ", env!("CARGO_PKG_VERSION")).to_owned())
    }

    pub fn update(state: &mut AppState, query: (), req: &api::HttpRequest) -> api::Result<String> {
        Ok("".to_owned())
    }

    fn user_path(info: Path<(u32, String)>) -> api::Result<String> {
        Ok(format!("Welcome {}! {}", info.1, info.0))
    }

    fn resource_test(req: &api::HttpRequest) -> api::Result<String> {
        Ok("resource_test".to_owned())
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        trace!("wiring API...");
        builder
            .public_scope()
            // .endpoint("v1/info", Self::info)
            .endpoint("v1/info_req", Self::info_req)
            .endpoint_mut("v1/update", Self::update)
            .with_scope(|scope| {
                scope
                    .resource("v1/coba", |r| r.method(Method::GET).h(Self::resource_test))
                    .resource("v1/coba2/{userid}/{username}", |r| {
                        r.method(Method::GET).with(Self::user_path)
                    })
            });
    }
}

