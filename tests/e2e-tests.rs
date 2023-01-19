mod testcases {
    #[macro_export]
    macro_rules! e2e_tests {
        ($($name:ident: $controller:expr,)*) => {
        $(
            mod $name {
                use e2e_testing::spin_controller::App;
                use e2e_testing::testcase::TestCase;
                use e2e_testing::asserts::assert_http_request;
                use anyhow::Result;

                #[tokio::test]
                async fn http_go_works() {
                    let controller = $controller;

                    fn checks(app: &App) -> Result<()> {
                        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello Fermyon!\n");
                    }

                    let tc = TestCase {
                        name: "http-go template".to_string(),
                        appname: "http-go-test".to_string(),
                        template: Some("http-go".to_string()),
                        assertions: checks,
                    };

                    match tc.run(&controller).await {
                        Ok(_) => assert!(true, "works"),
                        Err(err) => assert!(false, "{}", err),
                    }
                }

                #[tokio::test]
                async fn http_c_works() {
                    let controller = $controller;
                    fn checks(app: &App) -> Result<()> {
                        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello from WAGI/1\n");
                    }

                    let tc = TestCase {
                        name: "http-c template".to_string(),
                        appname: "http-c-test".to_string(),
                        template: Some("http-c".to_string()),
                        assertions: checks,
                    };
                    match tc.run(&controller).await {
                        Ok(_) => assert!(true, "works"),
                        Err(err) => assert!(false, "{}", err),
                    }
                }

                #[tokio::test]
                async fn http_rust_works() {
                    let controller = $controller;

                    fn checks(app: &App) -> Result<()> {
                        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, Fermyon");
                    }

                    let tc = TestCase {
                        name: "http-rust-template".to_string(),
                        appname: "http-rust-test".to_string(),
                        template: Some("http-rust".to_string()),
                        assertions: checks,
                    };

                    match tc.run(&controller).await {
                        Ok(_) => assert!(true, "works"),
                        Err(err) => assert!(false, "{}", err),
                    }
                }

                #[tokio::test]
                async fn http_zig_works() {
                    let controller = $controller;

                    fn checks(app: &App) -> Result<()> {
                        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello World!\n");
                    }

                    let tc = TestCase {
                        name: "http-zig-template".to_string(),
                        appname: "http-zig-test".to_string(),
                        template: Some("http-zig".to_string()),
                        assertions: checks,
                    };

                    match tc.run(&controller).await {
                        Ok(_) => assert!(true, "works"),
                        Err(err) => assert!(false, "{}", err),
                    }
                }

                #[tokio::test]
                async fn http_grain_works() {
                    let controller = $controller;

                    fn checks(app: &App) -> Result<()> {
                        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, World\n");
                    }

                    let tc = TestCase {
                        name: "http-grain-template".to_string(),
                        appname: "http-grain-test".to_string(),
                        template: Some("http-grain".to_string()),
                        assertions: checks,
                    };

                    match tc.run(&controller).await {
                        Ok(_) => assert!(true, "works"),
                        Err(err) => assert!(false, "{}", err),
                    }
                }
            }
        )*
        }
    }

    #[cfg(test)]
    mod spinup_tests {
        e2e_tests! {
            spin_up: e2e_testing::spin_controller::SpinUp{},
        }
    }

    #[cfg(test)]
    mod fermyon_cloud_tests {
        e2e_tests! {
            fermyon_cloud: e2e_testing::fc_controller::FermyonCloud{},
        }
    }
}
