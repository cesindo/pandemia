//! Core implementasi untuk Service user
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth, models,
    prelude::*,
};

use crate::api::user::{PrivateApi, PublicApi};

/// Core basis service pandemia.
pub struct UserService {}

impl UserService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for UserService {
    fn name(&self) -> &'static str {
        "user"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().link(PublicApi::wire);
        builder.private_scope().link(PrivateApi::wire);
    }
}
