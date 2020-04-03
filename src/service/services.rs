//! Service helper
//!
//! Here you can implement service as simple write one line
//! this done by using impl_service macro, example:
//!
//! `impl_service!(UserService, user);`
//!
//! You need to implement api interface for the above service inside `api` module.
//!
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth, models,
    prelude::*,
    service::{AuthService, SystemService},
};

macro_rules! impl_service {
    // Gunakan macro ini untuk mendeklarasikan service rest API dengan multi versi,
    // contoh `/v1` dan `/v2`.
    // Contoh penggunaan:
    //
    //     impl_service!(FeedService, feed, [PublicApi, PublicApiV2], [PrivateApi]);
    //
    ($service_name:ident, $api_name:ident, [ $($endp_st_pub:ident,)* ], [ $($endp_st_priv:ident,)* ]) => {
        mod $api_name {
            use crate::api::$api_name::{ $($endp_st_pub,)* $($endp_st_priv,)* };

            /// Core basis service racta.
            pub struct $service_name {}

            impl $service_name {
                #[doc(hidden)]
                pub fn new() -> Box<Self> {
                    Box::new(Self {})
                }
            }

            impl crate::service::Service for $service_name {
                fn name(&self) -> &'static str {
                    stringify!($api_name)
                }

                fn wire_api(&self, builder: &mut crate::api::ServiceApiBuilder) {
                    $(builder.public_scope().link($endp_st_pub::wire);)*
                    $(builder.private_scope().link($endp_st_priv::wire);)*
                }
            }
        }

        pub use $api_name::$service_name;
    };

    ($service_name:ident, $api_name:ident, [ $($endp_st_pub:ident),* ],  [ $($endp_st_priv:ident),* ]) => {
        impl_service!($service_name, $api_name, [ $($endp_st_pub,)* ], [ $($endp_st_priv,)* ]);
    };

    ($service_name:ident, $api_name:ident) => {
        impl_service!($service_name, $api_name, [PublicApi], [PrivateApi]);
    };
}

// Example implementing service using macro:
// impl_service!(UserService, user);
impl_service!(AdminService, admin);
impl_service!(PandemiaService, pandemia);
impl_service!(FeedService, feed);
impl_service!(MapAreaService, map_area);

/// Initialize and load services
pub fn load_services() -> Vec<Box<dyn Service>> {
    vec![
        AuthService::new(),
        AdminService::new(),
        PandemiaService::new(),
        FeedService::new(),
        SystemService::new(),
        UserService::new(),
        MapAreaService::new(),
    ]
}
