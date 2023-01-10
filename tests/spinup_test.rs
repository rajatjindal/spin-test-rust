use e2e_testing::spin_controller::SpinUp;

#[cfg(test)]
mod spinup_tests {
    use super::*;

    #[tokio::test]
    async fn holy_grail_test() {
        let tc = adder::http_go_works();
        let controller = SpinUp {};

        match tc.run(&controller).await {
            Ok(_) => assert!(true, "works"),
            Err(_) => assert!(false, "fails"),
        }
    }
}
