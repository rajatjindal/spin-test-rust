use crate::asserts::assert_http_request;
use crate::spin_controller::App;
use crate::testcase::TestCase;
use anyhow::Result;

#[macro_export]
macro_rules! e2e_tests {
    ($($name:ident: $controller:expr,)*) => {
    $(
        mod $name {
            use super::*;
            use e2e_testing::testcases;

            #[tokio::test]
            async fn http_go_works() {
                let controller = $controller;
                let tc = testcases::http_go_testcase();
                match tc.run(&controller).await {
                    Ok(_) => assert!(true, "works"),
                    Err(err) => assert!(false, "{}", err),
                }
            }

            #[tokio::test]
            async fn http_c_works() {
                let controller = $controller;
                let tc = testcases::http_c_testcase();
                match tc.run(&controller).await {
                    Ok(_) => assert!(true, "works"),
                    Err(err) => assert!(false, "{}", err),
                }
            }


        }
    )*
    }
}

pub fn http_go_testcase() -> TestCase {
    fn checks(app: &App) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello Fermyon!\n");
    }

    return TestCase {
        name: "http-go template".to_string(),
        appname: "http-go-test".to_string(),
        template: Some("http-go".to_string()),
        assertions: checks,
    };
}

pub fn http_c_testcase() -> TestCase {
    fn checks(app: &App) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello from WAGI/1\n");
    }

    return TestCase {
        name: "http-c template".to_string(),
        appname: "http-c-test".to_string(),
        template: Some("http-c".to_string()),
        assertions: checks,
    };
}

pub fn http_rust_works() -> TestCase {
    fn checks(app: &App) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, Fermyon");
    }

    return TestCase {
        name: "http-rust-template".to_string(),
        appname: "http-rust-test".to_string(),
        template: Some("http-rust".to_string()),
        assertions: checks,
    };
}

pub fn http_zig_works() -> TestCase {
    fn checks(app: &App) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello World!\n");
    }

    return TestCase {
        name: "http-zig-template".to_string(),
        appname: "http-zig-test".to_string(),
        template: Some("http-zig".to_string()),
        assertions: checks,
    };
}

pub fn http_grain_works() -> TestCase {
    fn checks(app: &App) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, World\n");
    }

    return TestCase {
        name: "http-grain-template".to_string(),
        appname: "http-grain-test".to_string(),
        template: Some("http-grain".to_string()),
        assertions: checks,
    };
}
