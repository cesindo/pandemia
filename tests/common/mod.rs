#![allow(dead_code, unused_variables)]

use pandemia_testkit::TestKit;

pub mod prelude {
    pub use super::{create_testkit, setup};
    pub use pandemia::api::{ApiResult, ErrorCode};
    pub use pandemia_testkit::{TestHelper, TestKit, TestKitApi};
}

pub fn setup() {
    let _ = env_logger::try_init();
}

pub fn create_testkit() -> TestKit {
    setup();
    TestKit::new()
}
