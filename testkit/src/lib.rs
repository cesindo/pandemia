#![allow(unused_imports)]

extern crate actix_web;
extern crate pandemia;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate diesel;

extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate serde_urlencoded;
#[macro_use]
extern crate lazy_static;

use actix_web::{
    http::{header::HeaderValue, HeaderMap},
    test::TestServer,
    App,
};
use reqwest::{Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::{env, fmt};

use pandemia::{
    api::{self, ApiAccess, ApiAggregator},
    service, ID,
};

pub use pandemia::api::{types::*, user::types::User};

pub mod helper;

pub use crate::helper::{ApiHelper, TestHelper};

/// Kind of API service.
///
#[derive(Debug, Clone, Copy)]
pub enum ApiKind {
    /// `api/system` endpoints
    System,
    /// `api/auth` endpoints. Mengarah ke servis [Auth].
    Auth,
    /// `api/user` endpoints. Mengarah ke servis [User].
    User,
    /// Gunakan ini apabila ada servis khusus (user).
    Service(&'static str),
}

impl fmt::Display for ApiKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiKind::System => write!(f, "api/system"),
            ApiKind::Auth => write!(f, "api/auth"),
            ApiKind::User => write!(f, "api/user"),
            ApiKind::Service(name) => write!(f, "api/{}", name),
        }
    }
}

#[derive(Clone)]
pub struct TestKit {
    pub accessor_user_id: Option<ID>,
    pub test_server_url: String,
}

impl TestKit {
    pub fn new() -> Self {
        create_test_server();
        let server_url = SERVER_URL.clone();
        let server_url = server_url.lock().unwrap();
        let test_server_url = server_url.to_owned();
        Self {
            // accessor_user: None,
            accessor_user_id: None,
            test_server_url,
        }
    }

    pub fn set_accessor(&mut self, accessor_id: ID) {
        self.accessor_user_id = Some(accessor_id);
    }

    pub fn api(&self) -> TestKitApi {
        let mut tapi = TestKitApi::new(self, self.test_server_url.to_owned());
        match &self.accessor_user_id {
            Some(_) => tapi.authorize_user(),
            None => (),
        }
        tapi
    }

    pub fn helper(&self) -> TestHelper {
        TestHelper::new(self)
    }

    pub fn api_helper(&self) -> ApiHelper {
        ApiHelper::new(self)
    }
}

pub struct TestKitApi {
    testkit: TestKit,
    // test_server: TestServer,
    test_client: Client,
    test_server_url: String,
}

impl TestKitApi {
    pub fn new(testkit: &TestKit, test_server_url: String) -> Self {
        TestKitApi {
            testkit: testkit.clone(),
            test_client: Client::new(),
            test_server_url,
        }
    }

    /// Creates a requests builder for the public API scope.
    pub fn public(&self, kind: impl fmt::Display) -> RequestBuilder {
        RequestBuilder::new(
            self.test_server_url.to_owned(),
            &self.test_client,
            ApiAccess::Public,
            kind.to_string(),
        )
    }

    /// Creates a requests builder for the private API scope.
    pub fn private(&self, kind: impl fmt::Display) -> RequestBuilder {
        RequestBuilder::new(
            self.test_server_url.to_owned(),
            &self.test_client,
            ApiAccess::Private,
            kind.to_string(),
        )
    }

    /// Cara pintas untuk meng-otorisasi User,
    /// atau dengan kata lain me-login-kan sehingga
    /// nanti http client akan meng-embed X-Access-Token secara otomatis.
    pub fn authorize(&mut self, user_id: ID) {
        let mut headers = HeaderMap::new();
        let token = self
            .testkit
            .helper()
            .gen_access_token_for(user_id)
            .expect("Cannot generate access token");
        headers.insert("X-Access-Token", HeaderValue::from_str(&token.token).unwrap());
        self.test_client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Cannot build http client");
    }

    /// Cara pintas untuk meng-otorisasi User,
    /// atau dengan kata lain me-login-kan sehingga
    /// nanti http client akan meng-embed X-Access-Token secara otomatis.
    pub fn authorize_user(&mut self) {
        let mut headers = HeaderMap::new();
        let token = self
            .testkit
            .helper()
            .gen_access_token_for(self.testkit.accessor_user_id.expect("No accessor user id"))
            .expect("Cannot generate user access token");
        headers.insert("X-Access-Token", HeaderValue::from_str(&token.token).unwrap());
        self.test_client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Cannot build http client");
    }

    /// Assert json result from API,
    /// akan gagal apabila hasil dari ApResult berisi kode error / status="error".
    pub fn assert_success(&self, rv: &serde_json::Value) {
        assert_eq!(rv, &json!({"code": 0, "status":"success", "description":""}));
    }
}

/// An HTTP requests builder. This type can be used to send requests to
/// the appropriate `TestKitApi` handlers.
pub struct RequestBuilder<'a, 'b, Q = ()>
where
    Q: 'b,
{
    test_server_url: String,
    test_client: &'a Client,
    access: ApiAccess,
    prefix: String,
    query: Option<&'b Q>,
}

impl<'a, 'b, Q> fmt::Debug for RequestBuilder<'a, 'b, Q>
where
    Q: 'b + fmt::Debug + Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("RequestBuilder")
            .field("access", &self.access)
            .field("prefix", &self.prefix)
            .field("query", &self.query)
            .finish()
    }
}

impl<'a, 'b, Q> RequestBuilder<'a, 'b, Q>
where
    Q: 'b + Serialize,
{
    fn new(test_server_url: String, test_client: &'a Client, access: ApiAccess, prefix: String) -> Self {
        RequestBuilder {
            test_server_url,
            test_client,
            access,
            prefix,
            query: None,
        }
    }

    /// Sets a query data of the current request.
    pub fn query<T>(&'a self, query: &'b T) -> RequestBuilder<'a, 'b, T> {
        RequestBuilder {
            test_server_url: self.test_server_url.clone(),
            test_client: self.test_client,
            access: self.access,
            prefix: self.prefix.clone(),
            query: Some(query),
        }
    }

    /// Sends a get request to the testing API endpoint and decodes response as
    /// the corresponding type.
    pub fn get<R>(&self, endpoint: &str) -> api::Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        let params = self
            .query
            .as_ref()
            .map(|query| {
                format!(
                    "?{}",
                    serde_urlencoded::to_string(query).expect("Unable to serialize query.")
                )
            })
            .unwrap_or_default();
        let url = format!(
            "{url}{access}/{prefix}/{endpoint}{query}",
            url = self.test_server_url,
            access = format!("{}", self.access).to_lowercase(),
            prefix = self.prefix,
            endpoint = endpoint,
            query = params
        );

        trace!("GET {}", url);

        let response = self.test_client.get(&url).send().expect("Unable to send request");
        Self::response_to_api_result(response)
    }

    /// Sends a post request to the testing API endpoint and decodes response as
    /// the corresponding type.
    pub fn post<R>(&self, endpoint: &str) -> api::Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        let url = format!(
            "{url}{access}/{prefix}/{endpoint}",
            url = self.test_server_url,
            access = format!("{}", self.access).to_lowercase(),
            prefix = self.prefix,
            endpoint = endpoint
        );

        trace!("POST {}", url);

        let builder = self.test_client.post(&url);
        let builder = if let Some(ref query) = self.query.as_ref() {
            trace!("Body: {}", serde_json::to_string_pretty(&query).unwrap());
            builder.json(query)
        } else {
            builder.json(&serde_json::Value::Null)
        };
        let response = builder.send().expect("Unable to send request");
        Self::response_to_api_result(response)
    }

    /// Converts reqwest Response to api::Result.
    fn response_to_api_result<R>(mut response: Response) -> api::Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        trace!("Response status: {}", response.status());

        fn extract_description(body: &str) -> Option<String> {
            trace!("Error: {}", body);
            match serde_json::from_str::<serde_json::Value>(body).ok()? {
                serde_json::Value::Object(ref object) if object.contains_key("description") => {
                    Some(object["description"].as_str()?.to_owned())
                }
                serde_json::Value::String(string) => Some(string),
                _ => None,
            }
        }

        fn error(mut response: Response) -> String {
            let body = response.text().expect("Unable to get response text");
            extract_description(&body).unwrap_or(body)
        }

        match response.status() {
            StatusCode::OK => Ok({
                let body = response.text().expect("Unable to get response text");
                eprintln!("error body: {}", body);
                serde_json::from_str(&body).expect("Unable to deserialize body")
            }),
            StatusCode::FORBIDDEN => Err(api::Error::Unauthorized),
            StatusCode::BAD_REQUEST => Err(api::Error::BadRequest(400, error(response))),
            StatusCode::NOT_FOUND => Err(api::Error::NotFound(404, error(response))),
            s if s.is_server_error() => {
                Err(api::Error::InternalError(500, format_err!("{}", error(response))))
            }
            s => {
                let body = response.text().expect("Unable to get response text");
                eprintln!("error body: {}", body);
                panic!(
                    "Received non-error response status: {} ({})",
                    s.as_u16(),
                    error(response)
                )
            }
        }
    }
}

pub fn setup() {
    let _ = env_logger::try_init();
    env::set_var(
        "DATABASE_URL",
        env::var("DATABASE_TEST_URL").expect("No DATABASE_TEST_URL"),
    );
}

use std::sync::{mpsc::channel, Arc, Mutex};
use std::{thread, time::Duration};

lazy_static! {
    static ref SERVER_URL: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
}

pub fn create_test_server() {
    setup();

    let (tx, rx) = channel();

    let server_url = SERVER_URL.clone();
    thread::spawn(move || {
        let mut server_url = server_url.lock().unwrap();

        if !server_url.is_empty() {
            // println!("SERVER_URL ALREADY DEFINED #2");
            tx.send(0).unwrap();
            return;
        }

        let services = service::load_services();

        let agg = ApiAggregator::new(services);

        let server = TestServer::with_factory(move || {
            let state = api::AppState::new();
            App::with_state(state.clone())
                .scope("public/api", |scope| {
                    trace!("Create public API");
                    agg.extend(ApiAccess::Public, scope)
                })
                .scope("private/api", |scope| {
                    trace!("Create private API");
                    agg.extend(ApiAccess::Private, scope)
                })
        });

        info!("Test server created on {}", server.addr());

        *server_url = server.url("");

        debug!("TEST SERVER URL: {}", &*server_url);

        drop(server_url);

        tx.send(1).expect("Cannot send tx");

        for _ in 0..1000 {
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::sleep(Duration::from_millis(50));

    // tunggu sampai server_url telah terisi
    debug!("Waiting for test server to become ready...");
    debug!("Test Server READY! {}", rx.recv().unwrap());
}
