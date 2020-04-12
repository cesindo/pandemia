//! Modular & extendable Service interface

use actix_web::{http::Method, App, AsyncResponder, Error, Path, Result};
use serde_json::Value as JsonValue;

use crate::api;
use crate::api::*;
use crate::service::Service;

/// Service contoh, kamu bisa mencontoh bagaimana caranya membuat service
/// dengan melihat kode [SystemService] ini.
pub struct SystemService;

impl SystemService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for SystemService {
    fn name(&self) -> &'static str {
        "system"
    }
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().link(PublicApi::wire);
    }
}

/// Contoh API public untuk service contoh [[SystemService]].
struct PublicApi {}

#[api_group("System", "public", base = "/system/v1")]
impl PublicApi {
    /// Get build information.
    #[api_endpoint(path = "/info", auth = "optional")]
    pub fn info(state: &AppState, query: ()) -> JsonValue {
        Ok(json!({ "version": env!("CARGO_PKG_VERSION"), 
                "build": env!("BUILD_INFO"), "git": env!("GIT_REV") }))
    }
}
