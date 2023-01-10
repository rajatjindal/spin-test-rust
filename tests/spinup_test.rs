use e2e_testing::fc_controller::FermyonCloud;
// use e2e_testing::spin_controller::SpinUp;
use e2e_testing::testcases;

#[cfg(test)]
mod spinup_tests {
    use super::*;

    #[tokio::test]
    async fn holy_grail_test() {
        let tc = testcases::http_go_works();
        let controller = FermyonCloud {};

        match tc.run(&controller).await {
            Ok(_) => assert!(true, "works"),
            Err(err) => assert!(false, "{}", err),
        }
    }
}
