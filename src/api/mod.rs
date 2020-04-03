//! Module inti yang berkaitan dengan kebutuhan pembuatan rest API.
//!

use actix_web::{
    actix::System,
    http::header,
    middleware::{self, cors::Cors},
    server::HttpServer,
    AsyncResponder, FromRequest, HttpMessage, HttpResponse, Query,
};
use diesel::{pg::PgConnection, prelude::*};
use futures::future::{Future, IntoFuture};
use regex::Regex;

mod error;
mod with;

pub mod admin;
pub mod feed;
pub mod map_area;
pub mod pandemia;
pub mod types;
pub mod user;

use self::with::{Immutable, ImmutableReq, Mutable, MutableReq, NamedWith, With};
pub use self::{error::Error, with::Result};
pub use crate::{auth, error::ErrorCode, user_dao};

use crate::eventstream::{self, Event};
use crate::{db, service::Service};

use std::{
    collections::BTreeMap,
    convert::From,
    env, fmt,
    marker::PhantomData,
    sync::{mpsc, Arc},
    thread,
};

/// Jenis penanda akses API, kita bagikan menjadi 2 macam:
///
/// * Public
/// * Private
///
/// Public adalah apabila kita ingin akses API-nya boleh digunakan oleh publik.
/// Sementara Private adalah apabila kita ingin akses API-nya hanya untuk internal,
/// nantinya masing-masing akses ini di-serve pada port yang berbeda
/// sehingga perlu dilakukan settingan firewall oleh system administrator
/// agar port untuk private API hanya boleh diakses dari jaringan internal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiAccess {
    /// Penanda untuk akses publik
    Public,

    /// Penanda untuk akses privat
    Private,
}

impl fmt::Display for ApiAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiAccess::Public => f.write_str("Public"),
            ApiAccess::Private => f.write_str("Private"),
        }
    }
}

use serde::{de::DeserializeOwned, Serialize};

/// Struktur data ketika pemanggilan api sukses.
#[derive(Serialize, Deserialize)]
pub struct ApiResult<T> {
    /// Error code untuk memberikan informasi hasil pengembalian,
    /// apabila tidak ada error terjadi maka code harus berisi 0.
    pub code: i32,

    /// Status bisa berisi: "success" atau "error".
    pub status: String,

    /// Deskripsi error apabila terjadi error.
    pub description: String,

    /// Result data.
    pub result: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    #[doc(hidden)]
    pub fn new(code: i32, status: String, description: String, result: Option<T>) -> Self {
        ApiResult {
            code,
            status,
            description,
            result,
        }
    }

    /// Buat hasil sukses
    pub fn success(result: T) -> Self {
        Self::new(0, "success".to_owned(), "".to_owned(), Some(result))
    }
}

impl ApiResult<()> {
    /// Buat hasil error
    pub fn error(code: i32, description: String) -> ApiResult<()> {
        // Self::new(code, "error".to_owned(), description, None::<_>)
        ApiResult {
            code,
            status: "error".to_owned(),
            description,
            result: None::<()>,
        }
    }
}

/// Defines an object that could be used as an API backend.
///
/// This trait is used to implement an API backend for Exonum.
pub trait ServiceApiBackend: Sized {
    /// Concrete endpoint handler in the backend.
    type Handler;
    /// Concrete backend API builder.
    type Backend;

    /// Adds the given endpoint handler to the backend.
    fn endpoint<N, Q, I, R, F, E, K>(&mut self, name: N, endpoint: E) -> &mut Self
    where
        N: Into<String>,
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        E: Into<With<Q, I, R, F>>,
        Self::Handler: From<NamedWith<Q, I, R, F, K>>,
    {
        let named_with = NamedWith::new(name, endpoint);
        self.raw_handler(Self::Handler::from(named_with))
    }

    /// Adds the given mutable endpoint handler to the backend.
    fn endpoint_mut<N, Q, I, R, F, E, K>(&mut self, name: N, endpoint: E) -> &mut Self
    where
        N: Into<String>,
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        E: Into<With<Q, I, R, F>>,
        Self::Handler: From<NamedWith<Q, I, R, F, K>>,
    {
        let named_with = NamedWith::new(name, endpoint);
        self.raw_handler(Self::Handler::from(named_with))
    }

    /// Adds the raw endpoint handler for the given backend.
    fn raw_handler(&mut self, handler: Self::Handler) -> &mut Self;

    /// Binds API handlers to the given backend.
    fn wire(&self, output: Self::Backend) -> Self::Backend;
}

/// Type alias for the concrete `actix-web` HTTP response.
pub type FutureResponse = actix_web::FutureResponse<HttpResponse, actix_web::Error>;
/// Type alias for the concrete `actix-web` HTTP request.
pub type HttpRequest = actix_web::HttpRequest<AppState>;
/// Type alias for the inner `actix-web` HTTP requests handler.
pub type RawHandler = dyn Fn(HttpRequest) -> FutureResponse + 'static + Send + Sync;
/// Type alias for the `actix-web::App` with the `AppState`.
pub type App = actix_web::App<AppState>;
/// Type alias for actix `Scope` with `AppState`.
pub type Scope = actix_web::Scope<AppState>;
/// Type alias for the `actix-web::App` configuration.
pub type AppConfig = Arc<dyn Fn(App) -> App + 'static + Send + Sync>;

/// Raw `actix-web` backend requests handler.
#[derive(Clone)]
pub struct RequestHandler {
    /// Endpoint name.
    pub name: String,
    /// Endpoint HTTP method.
    pub method: actix_web::http::Method,
    /// Inner handler.
    pub inner: Arc<RawHandler>,
}

impl fmt::Debug for RequestHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RequestHandler")
            .field("name", &self.name)
            .field("method", &self.method)
            .finish()
    }
}

/// API builder for the `actix-web` backend.
#[derive(Debug, Clone, Default)]
pub struct ApiBuilder {
    handlers: Vec<RequestHandler>,
}

impl ApiBuilder {
    /// Constructs a new backend builder instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ServiceApiBackend for ApiBuilder {
    type Handler = RequestHandler;
    type Backend = actix_web::Scope<AppState>;

    fn raw_handler(&mut self, handler: Self::Handler) -> &mut Self {
        self.handlers.push(handler);
        self
    }

    fn wire(&self, mut output: Self::Backend) -> Self::Backend {
        for handler in self.handlers.clone() {
            let inner = handler.inner;
            output = output.route(&handler.name, handler.method.clone(), move |request| {
                inner(request)
            });
        }
        output
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, Immutable>> for RequestHandler
where
    F: for<'r> Fn(&'r AppState, Q) -> Result<I> + 'static + Send + Sync + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, Immutable>) -> Self {
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let context = request.state();
            let future = Query::from_request(&request, &Default::default())
                .map(|query: Query<Q>| query.into_inner())
                .or_else(map_error)
                .and_then(|query| handler(context, query).map_err(From::from))
                .and_then(|value| Ok(map_ok(value, &request)))
                .into_future();
            Box::new(future)
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::GET,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, ImmutableReq>> for RequestHandler
where
    F: for<'r> Fn(&'r AppState, Q, &HttpRequest) -> Result<I> + 'static + Send + Sync + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, ImmutableReq>) -> Self {
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let context = request.state();
            let future = Query::from_request(&request, &Default::default())
                .map(|query: Query<Q>| query.into_inner())
                .or_else(map_error)
                .and_then(|query| handler(context, query, &request).map_err(From::from))
                .and_then(|value| Ok(map_ok(value, &request)))
                .into_future();
            Box::new(future)
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::GET,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

// Me-mapping pengembalian `Ok(())` menjadi format [ApiResult].
#[inline]
fn map_ok<I: Serialize>(value: I, request: &HttpRequest) -> HttpResponse {
    let headers = request.headers();
    match serde_json::to_string(&value) {
        Ok(body) => {
            let contains = headers.contains_key(header::CONTENT_TYPE);

            if body == "null" {
                HttpResponse::Ok()
                    .header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
                    .json(ApiResult::<()>::new(
                        ErrorCode::NoError as i32,
                        "".to_string(),
                        "".to_string(),
                        None,
                    ))
            } else {
                HttpResponse::Ok()
                    .header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
                    .body(body)
            }
        }
        Err(e) => panic!("cannot serialize response"),
    }
}

// Me-mapping pengembalian error ketika parsing query agar bisa ditampilkan ke client.
#[inline]
fn map_error<I: DeserializeOwned + 'static, E>(e: E) -> ::std::result::Result<I, actix_web::Error>
where
    E: Into<actix_web::error::Error> + fmt::Display,
{
    // @TODO(*): Regex ini mungkin perlu dibuat lazy_static?
    let re = Regex::new(r"missing field `(.*?)`").unwrap();
    let err_desc = format!("{}", e);
    debug!("err_desc: {}", err_desc);
    let mut iter = re.captures_iter(&err_desc);
    if let Some(field) = iter.next() {
        Err(actix_web::Error::from(Error::InvalidParameter(
            ErrorCode::InvalidParameter as i32,
            format!("No `{}` parameter", &field[1]),
        )))
    } else {
        Err(actix_web::Error::from(Error::InvalidParameter(
            ErrorCode::SerializeDeserializeError as i32,
            "Invalid parameter data".to_owned(),
        )))
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, Mutable>> for RequestHandler
where
    F: for<'r> Fn(&'r mut AppState, Q) -> Result<I> + 'static + Send + Sync + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + PartialEq + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, Mutable>) -> Self {
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let handler = handler.clone();
            let mut context = request.state().clone();
            request
                .json()
                // .from_err()
                .or_else(map_error)
                .and_then(move |query: Q| {
                    handler(&mut context, query)
                        .map(|v| map_ok(v, &request))
                        .map_err(From::from)
                })
                .responder()
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::POST,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

impl<Q, I, F> From<NamedWith<Q, I, Result<I>, F, MutableReq>> for RequestHandler
where
    F: for<'r> Fn(&'r mut AppState, Q, &HttpRequest) -> Result<I> + 'static + Send + Sync + Clone,
    Q: DeserializeOwned + 'static,
    I: Serialize + 'static,
{
    fn from(f: NamedWith<Q, I, Result<I>, F, MutableReq>) -> Self {
        let handler = f.inner.handler;
        let index = move |request: HttpRequest| -> FutureResponse {
            let handler = handler.clone();
            let mut context = request.state().clone();

            request
                .json()
                .or_else(map_error)
                .and_then(move |query: Q| {
                    handler(&mut context, query, &request)
                        .map(|v| map_ok(v, &request))
                        .map_err(From::from)
                })
                .responder()
        };

        Self {
            name: f.name,
            method: actix_web::http::Method::POST,
            inner: Arc::from(index) as Arc<RawHandler>,
        }
    }
}

/// Just type alias for complex type
pub type ResourceFunc = Arc<Box<dyn Fn(Scope) -> Scope + Sync + Send + 'static>>;

/// Scope API
#[derive(Default, Clone)]
pub struct ServiceApiScope {
    pub(crate) actix_backend: ApiBuilder,
    pub(crate) resources: Vec<ResourceFunc>,
}

impl ServiceApiScope {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Menghubungkan API endpoints dari API endpoint holder.
    pub fn link<F>(&mut self, wire_func: F)
    where
        F: FnOnce(&mut ServiceApiScope) -> (),
    {
        wire_func(self)
    }

    fn endpoint_internal<Q, I, R, F, E, K>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: Into<FuncHandler<Func2<Q, R>>>,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, K>>,
    {
        self.actix_backend.endpoint(name, endpoint);
        self
    }

    /// Add endpoint with additional `HttpRequest` object as third parameter.
    pub fn endpoint<Q, I, R, F, E>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, ImmutableReq>>,
    {
        self.endpoint_internal(name, endpoint);
        self
    }

    fn endpoint_internal_mut<Q, I, R, F, E, K>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        // F: for<'r> Fn(&'r AppState, Q) -> R + 'static + Clone,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, K>>,
    {
        self.actix_backend.endpoint_mut(name, endpoint);
        self
    }

    /// Adds the given mutable endpoint handler to the API scope. These endpoints
    /// are designed for modification operations.
    ///
    /// With additional `HttpRequest` as third parameter.
    ///
    /// For now there is only web backend and it has the following requirements:
    ///
    /// - Query parameters should be decodable via `serde_json`.
    /// - Response items also should be encodable via `serde_json` crate.
    pub fn endpoint_mut<Q, I, R, F, E>(&mut self, name: &'static str, endpoint: E) -> &mut Self
    where
        Q: DeserializeOwned + 'static,
        I: Serialize + 'static,
        F: for<'r> Fn(&'r mut AppState, Q, &HttpRequest) -> R + 'static + Clone,
        E: Into<With<Q, I, R, F>>,
        RequestHandler: From<NamedWith<Q, I, R, F, MutableReq>>,
    {
        self.endpoint_internal_mut(name, endpoint);
        self
    }

    /// Mendaftarkan raw actix web handler. Berguna apabila kamu ingin
    /// menambahkan handler dengan spesifikasi kompleks yang hanya bisa
    /// dilakukan di level actix.
    ///
    /// # Example
    ///
    /// ```
    /// use actix_web::{http::Method, Path};
    /// use pandemia::api::{self, ServiceApiBuilder};
    ///
    /// let mut builder = ServiceApiBuilder::new();
    ///
    /// fn user_path(info: Path<(u32, String)>) -> api::Result<String> {
    ///    Ok(format!("Welcome {}! {}", info.1, info.0))
    /// }
    ///
    /// builder
    ///     .public_scope()
    ///     .with_scope(|scope| {
    ///         scope.resource("v1/coba2/{userid}/{username}", |r| {
    ///               r.method(Method::GET).with(user_path)
    ///         })
    ///     });
    /// ```
    pub fn with_scope<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Scope) -> Scope + Sync + Send + 'static,
    {
        {
            self.resources.push(Arc::new(Box::new(f)));
        }
        self
    }

    /// Returns a mutable reference to the underlying web backend.
    pub fn web_backend(&mut self) -> &mut ApiBuilder {
        &mut self.actix_backend
    }
}

/// API builder untuk build endpoint berdasarkan scope aksesnya
/// lihat juga [[ApiAccess]]
#[derive(Default, Clone)]
pub struct ServiceApiBuilder {
    public_scope: ServiceApiScope,
    private_scope: ServiceApiScope,
}

impl ServiceApiBuilder {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    /// Returns to a mutable reference to the public API builder.
    pub fn public_scope(&mut self) -> &mut ServiceApiScope {
        &mut self.public_scope
    }

    /// Returs to a mutable reference to the private API builder.
    pub fn private_scope(&mut self) -> &mut ServiceApiScope {
        &mut self.private_scope
    }
}

/// API Aggregator digunakan untuk meng-aggregate requirements untuk keperluan
/// serving rest API-nya.
#[derive(Clone)]
pub struct ApiAggregator {
    inner: BTreeMap<String, ServiceApiBuilder>,
}

impl ApiAggregator {
    /// Create new `ApiAggregator` instance
    pub fn new(services: Vec<Box<dyn Service>>) -> Self {
        let mut inner = BTreeMap::new();

        inner.insert("system".to_owned(), Self::system_api());

        let mut service_names: Vec<String> = vec![];

        inner.extend(services.iter().map(|service| {
            let prefix = service.name().to_string();

            if service_names.contains(&prefix) {
                panic!("Service with name `{}` already exists.", prefix);
            }

            let mut builder = ServiceApiBuilder::new();

            service.wire_api(&mut builder);

            service_names.push(prefix.to_owned());

            (prefix, builder)
        }));

        Self { inner }
    }

    #[inline]
    fn bind<'a, F>(items: F, mut scope: Scope) -> Scope
    where
        F: ::std::iter::IntoIterator<Item = (&'a str, &'a ServiceApiScope)>,
    {
        for item in items {
            scope = scope.nested(&item.0, move |scope| {
                let mut scope = item.1.actix_backend.wire(scope);
                let ress = item.1.resources.iter();
                for res in ress.as_ref() {
                    scope = res(scope)
                }
                scope
            });
        }
        scope
    }

    /// Untuk meng-extend scope dengan endpoint yang kita inginkan.
    ///
    /// # Arguments
    ///
    /// * `access` - API access kind.
    /// * `scope` - Actix scope instance.
    pub fn extend(&self, access: ApiAccess, scope: Scope) -> Scope {
        match access {
            ApiAccess::Public => {
                let items = self
                    .inner
                    .iter()
                    .map(|(name, builder)| (name.as_ref(), &builder.public_scope));

                Self::bind(items, scope)
            }
            ApiAccess::Private => {
                let items = self
                    .inner
                    .iter()
                    .map(|(name, builder)| (name.as_ref(), &builder.private_scope));

                Self::bind(items, scope)
            }
        }
    }

    /// Build system API
    #[allow(clippy::let_and_return)]
    pub fn system_api() -> ServiceApiBuilder {
        let builder = ServiceApiBuilder::new();
        // TODO: code here
        builder
    }
}

use db::{DbConn, DbConnMan};

/// State/context yang akan selalu bisa diakses dari handler
/// state ini berisi beberapa object yang mungkin sering digunakan
/// seperti DB connection.
#[derive(Clone)]
pub struct AppState {
    db: DbConnMan,
}

impl AppState {
    #[doc(hidden)]
    pub fn new() -> AppState {
        AppState { db: db::clone() }
    }

    /// Get Backend DB connection
    pub fn db(&self) -> DbConn {
        self.db
            .get()
            .expect("cannot get DB connection from the r2d2 pool")
    }
}

#[doc(hidden)]
pub fn create_app(agg: &ApiAggregator, access: ApiAccess) -> App {
    let state = AppState::new();
    let mut app = App::with_state(state)
        .middleware(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
        .middleware(Cors::default());
    app = app.scope("api", |scope: Scope| agg.extend(access, scope));
    app
}

/// Stelan untuk server API yang akan dijalankan
/// ini memungkinkan kita menjalankan server untuk setiap akses pada listen address yang berbeda.
/// Sebagai contoh rest API untuk public ada di port 8000 dan untuk private ada di 9000.
#[derive(Clone)]
pub struct ApiServer {
    /// Akses untuk API ini.
    pub access: ApiAccess,

    /// Listen address untuk API ini.
    pub listen_address: String,
}

impl ApiServer {
    #[doc(hidden)]
    pub fn new(access: ApiAccess, listen_address: String) -> Self {
        Self {
            access,
            listen_address,
        }
    }
}

/// Konfigurasi untuk servis API
pub struct ServiceApiConfig {
    /// Daftar server API yang akan dijalankan.
    pub api_servers: Vec<ApiServer>,
}

impl ServiceApiConfig {
    #[doc(hidden)]
    pub fn new(api_servers: Vec<ApiServer>) -> Self {
        ServiceApiConfig { api_servers }
    }
}

/// Start API server berdasarkan konfigurasi yang ada.
pub fn start(agg: ApiAggregator, config: ServiceApiConfig) {
    let (system_tx, system_rx) = mpsc::channel();
    let (api_runtime_tx, api_runtime_rx) = mpsc::channel();

    let api_servers = config.api_servers.clone();

    let system_thread = thread::spawn(move || -> ::std::result::Result<(), failure::Error> {
        let system = System::new("http-server");

        let api_handlers = api_servers.iter().map(|api_server| {
            let access = api_server.access;
            let listen_address = api_server.listen_address.clone();
            let agg = agg.clone();

            println!("{} rest API serving at {}", access, listen_address);
            HttpServer::new(move || create_app(&agg, access))
                .bind(listen_address)
                .map(|server| server.start())
        });

        system_tx.send(System::current())?;
        for handler in api_handlers {
            api_runtime_tx.send(handler?)?;
        }

        trace!("starting server...");

        println!("\nSystem Ready.");

        eventstream::emit(Event::Startup());

        // Starts actix-web runtime.
        let code = system.run();

        trace!("Actix runtime finished with code {}", code);
        ensure!(
            code == 0,
            "Actix runtime finished with the non zero error code: {}",
            code
        );

        Ok(())
    });

    let api_servers = config.api_servers.clone();

    // Receives addresses of runtime items.
    let system = system_rx
        .recv()
        .map_err(|_| format_err!("Unable to receive actix system handle"));

    if let Err(e) = &system {
        error!("{}", e);
    }

    if let Err(er) = system_thread.join().unwrap() {
        eprintln!("ERROR: Cannot start server. {}", er);
    }

    println!("done.");
}
