use e2e_testing::testcase::TestCase;

pub fn http_go_works() -> TestCase {
    return TestCase {
        name: "http-go template".to_string(),
        appname: "http-go-test".to_string(),
        template: Some("http-go".to_string()),
        metadata_extractor: None,
    };
}
