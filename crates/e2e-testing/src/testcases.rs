use crate::asserts::assert_status;
use crate::spin_controller::App;
use crate::testcase::TestCase;
use anyhow::Result;

pub fn all_testcases() -> Vec<TestCase> {
    return vec![http_go_works()];
}

pub fn http_go_works() -> TestCase {
    fn checks(app: &App) -> Result<()> {
        println!("metadata version {}", app.metadata.version);
        return assert_status(app.metadata.base.as_str(), 200);
    }

    return TestCase {
        name: "http-go template".to_string(),
        appname: "http-go-test".to_string(),
        template: Some("http-go".to_string()),
        assertions: checks,
    };
}
