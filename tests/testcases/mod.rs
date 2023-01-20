use anyhow::Result;
use e2e_testing::asserts::assert_http_request;
use e2e_testing::controller::{AppInstance, Controller};
use e2e_testing::testcase::TestCase;

pub async fn http_go_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello Fermyon!\n");
    }

    let tc = TestCase {
        name: "http-go template".to_string(),
        appname: "http-go-test".to_string(),
        template: Some("http-go".to_string()),
        assertions: checks,
    };

    tc.run(controller).await.unwrap();
}

pub async fn http_c_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello from WAGI/1\n");
    }

    let tc = TestCase {
        name: "http-c template".to_string(),
        appname: "http-c-test".to_string(),
        template: Some("http-c".to_string()),
        assertions: checks,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_rust_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, Fermyon");
    }

    let tc = TestCase {
        name: "http-rust-template".to_string(),
        appname: "http-rust-test".to_string(),
        template: Some("http-rust".to_string()),
        assertions: checks,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_zig_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello World!\n");
    }

    let tc = TestCase {
        name: "http-zig-template".to_string(),
        appname: "http-zig-test".to_string(),
        template: Some("http-zig".to_string()),
        assertions: checks,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_grain_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, World\n");
    }

    let tc = TestCase {
        name: "http-grain-template".to_string(),
        appname: "http-grain-test".to_string(),
        template: Some("http-grain".to_string()),
        assertions: checks,
    };

    tc.run(controller).await.unwrap()
}
