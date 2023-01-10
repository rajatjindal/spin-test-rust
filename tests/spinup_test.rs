use e2e_testing::spin_controller::SpinUp;
use e2e_testing::testcase::TestCase;

#[cfg(test)]
mod spinup_tests {
    use super::*;

    #[test]
    fn http_go_works() {
        let tc = TestCase {
            name: "http-go template".to_string(),
            appname: "http-go-test".to_string(),
            template: Some("http-go".to_string()),
            metadata_extractor: None,
        };
        let controller = SpinUp {};

        match tc.run(&controller) {
            Ok(_) => assert!(true, "works"),
            Err(_) => assert!(false, "fails"),
        }
    }
}
