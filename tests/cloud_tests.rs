mod testcases;
use e2e_testing::cloud_controller::FermyonCloud;
use e2e_testing::controller::Controller;

const CONTROLLER: &dyn Controller = &FermyonCloud {};

mod cloud_tests {
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

    #[tokio::test]
    async fn http_ts_works() {
        testcases::http_ts_works(CONTROLLER).await
    }

    #[tokio::test]
    async fn http_js_works() {
        testcases::http_js_works(CONTROLLER).await
    }

    #[tokio::test]
    async fn assets_routing_works() {
        testcases::assets_routing_works(CONTROLLER).await
    }
}
