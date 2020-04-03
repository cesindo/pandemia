//! Kode servis untuk Pandemia
//!

use crate::api;
use crate::api::types::*;
use crate::api::*;
use crate::service::Service;
use actix_web::{http::Method, App, AsyncResponder, Error, Path, Result};

/// Main Pandemia service
pub struct PandemiaService;

impl Service for PandemiaService {
    fn name(&self) -> &'static str {
        "pandemia"
    }
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        // builder.public_scope().link(PublicApi1::wire);
        PublicApi1::wire(builder);
        builder.public_scope().link(api::pandemia::PublicApi::wire);
        builder.private_scope().link(api::pandemia::PrivateApi::wire);
        // builder.private_scope().link(PrivateApi::wire);
    }
}

/// Contoh API public untuk service contoh [[PandemiaService]].
struct PublicApi1 {}

impl PublicApi1 {
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
