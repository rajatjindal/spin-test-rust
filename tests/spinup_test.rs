use e2e_testing::spin_controller::SpinUp;
use e2e_testing::testcases;

#[cfg(test)]
mod spinup_tests {
    use super::*;

    macro_rules! run {
        ($name:expr, controller:expr, $tc:expr) => {
            #[test]
            fn $name() {
                let (controller, tc) = $value;
                match tc.run(&controller).await {
                    Ok(_) => assert!(true, "works"),
                    Err(err) => assert!(false, "{}", err),
                }
            }
        };
    }

    run!("name", SpinUp {}, testcase::http_go_works());
    // macro_rules! run {
    //     ($name:expr, $controller:expr, $tc:expr) => {
    //         #[tokio::test]
    //         fn $name() {
    //             let controller = $controller;
    //             let tc = $tc;
    //             match tc.run(&controller).await {
    //                 Ok(_) => assert!(true, "works"),
    //                 Err(err) => assert!(false, "{}", err),
    //             }
    //         }
    //     };
    // }

    // macro_rules! run {
    //     // macth like arm for macro
    //     ($name:expr, $controller:expr, $tc:expr) => {
    //         $(
    //             #[tokio::test]
    //             async fn $name() {
    //                 let controller = $controller;
    //                 let tc = $tc;

    //             }
    //         )*
    //     };
    // }

    #[tokio::test]
    async fn holy_grail_test() {
        let controller = SpinUp {};
        for tc in testcases::all_testcases().iter() {
            // run!(tc.appname, controller, tc);
            // match tc.run(&controller).await {
            //     Ok(_) => assert!(true, "works"),
            //     Err(err) => assert!(false, "{}", err),
            // }
        }
    }
}
