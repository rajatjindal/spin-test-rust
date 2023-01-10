use crate::asserts::assert_status;
use crate::spin_controller::App;
use crate::testcase::TestCase;

pub fn http_go_works() -> TestCase {
    fn checks(app: &App) {
        println!("metadata version {}", app.metadata.version);
        match assert_status(app.metadata.base.as_str(), 200) {
            Err(error) => panic!("assert failed {:?}", error),
            _ => (),
        }
    }

    return TestCase {
        name: "http-go template".to_string(),
        appname: "http-go-test".to_string(),
        template: Some("http-go".to_string()),
        assertions: checks,
    };
}
