use e2e_testing::spin_controller::{Controller, SpinUp};
use e2e_testing::testcase::TestCase;

#[cfg(test)]
mod spinup_tests {
    use super::*;

    #[test]
    fn http_go_works() {
        let _tc = TestCase {
            name: "http-go template".to_string(),
            appname: "http-go-test".to_string(),
            template: Some("http-go".to_string()),
            metadata_extractor: None,
        };

        let controller = SpinUp {};
        let result = controller.name();
        assert_eq!(result, "spin-up");
    }
}
