mod testcases;
use e2e_testing::spin_controller::{Controller, SpinUp};

const CONTROLLER: &dyn Controller = &SpinUp {};

mod spinup_tests {
    use super::testcases;
    use super::CONTROLLER;

    #[tokio::test]
    async fn http_go_works() {
        testcases::http_go_works(CONTROLLER).await
    }

    #[tokio::test]
    async fn http_c_works() {
        testcases::http_c_works(CONTROLLER).await
    }

    #[tokio::test]
    async fn http_rust_works() {
        testcases::http_rust_works(CONTROLLER).await
    }

    #[tokio::test]
    async fn http_zig_works() {
        testcases::http_zig_works(CONTROLLER).await
    }

    #[tokio::test]
    async fn http_grain_works() {
        testcases::http_grain_works(CONTROLLER).await
    }
}
