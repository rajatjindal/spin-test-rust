use anyhow::Result;
use e2e_testing::asserts::assert_http_request;
use e2e_testing::controller::{AppInstance, Controller};
use e2e_testing::testcase::TestCase;

pub async fn http_go_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello Fermyon!\n");
    }

    let tc = TestCase {
        name: "http-go template".to_string(),
        appname: "http-go-test".to_string(),
        template: Some("http-go".to_string()),
        template_install_args: None,
        assertions: checks,
        plugins: None,
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: None,
    };

    tc.run(controller).await.unwrap();
}

pub async fn http_c_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello from WAGI/1\n");
    }

    let tc = TestCase {
        name: "http-c template".to_string(),
        appname: "http-c-test".to_string(),
        template: Some("http-c".to_string()),
        template_install_args: None,
        assertions: checks,
        plugins: None,
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: None,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_rust_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, Fermyon");
    }

    let tc = TestCase {
        name: "http-rust-template".to_string(),
        appname: "http-rust-test".to_string(),
        template: Some("http-rust".to_string()),
        template_install_args: None,
        assertions: checks,
        plugins: None,
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: None,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_zig_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello World!\n");
    }

    let tc = TestCase {
        name: "http-zig-template".to_string(),
        appname: "http-zig-test".to_string(),
        template: Some("http-zig".to_string()),
        template_install_args: None,
        assertions: checks,
        plugins: None,
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: None,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_grain_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello, World\n");
    }

    let tc = TestCase {
        name: "http-grain-template".to_string(),
        appname: "http-grain-test".to_string(),
        template: Some("http-grain".to_string()),
        template_install_args: None,
        assertions: checks,
        plugins: None,
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: None,
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_ts_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello from TS-SDK");
    }

    let tc = TestCase {
        name: "http-ts-template".to_string(),
        appname: "http-ts-test".to_string(),
        template: Some("http-ts".to_string()),
        template_install_args: Some(vec![
            "--git".to_string(),
            "https://github.com/fermyon/spin-js-sdk".to_string(),
            "--update".to_string(),
        ]),
        assertions: checks,
        plugins: Some(vec!["js2wasm".to_string()]),
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: Some(vec![vec!["npm".to_string(), "install".to_string()]]),
    };

    tc.run(controller).await.unwrap()
}

pub async fn http_js_works(controller: &dyn Controller) {
    fn checks(app: &AppInstance) -> Result<()> {
        return assert_http_request(app.metadata.base.as_str(), 200, &[], "Hello from JS-SDK");
    }

    let tc = TestCase {
        name: "http-js-template".to_string(),
        appname: "http-js-test".to_string(),
        template: Some("http-js".to_string()),
        template_install_args: Some(vec![
            "--git".to_string(),
            "https://github.com/fermyon/spin-js-sdk".to_string(),
            "--update".to_string(),
        ]),
        assertions: checks,
        plugins: Some(vec!["js2wasm".to_string()]),
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: Some(vec![vec!["npm".to_string(), "install".to_string()]]),
    };

    tc.run(controller).await.unwrap()
}

pub async fn assets_routing_works(controller: &dyn Controller) {
    fn get_url(base: &str, path: &str) -> String {
        return format!("{}{}", base, path);
    }

    fn checks(app: &AppInstance) -> Result<()> {
        assert_http_request(
            get_url(app.metadata.base.as_str(), "/static/thisshouldbemounted/1").as_str(),
            200,
            &[],
            "1\n",
        )?;

        assert_http_request(
            get_url(app.metadata.base.as_str(), "/static/thisshouldbemounted/2").as_str(),
            200,
            &[],
            "2\n",
        )?;

        assert_http_request(
            get_url(app.metadata.base.as_str(), "/static/thisshouldbemounted/3").as_str(),
            200,
            &[],
            "3\n",
        )?;

        assert_http_request(
            get_url(app.metadata.base.as_str(), "/static/donotmount/a").as_str(),
            404,
            &[],
            "Not Found",
        )?;

        assert_http_request(
            get_url(
                app.metadata.base.as_str(),
                "/static/thisshouldbemounted/thisshouldbeexcluded/4",
            )
            .as_str(),
            404,
            &[],
            "Not Found",
        )?;

        Ok(())
    }

    let tc = TestCase {
        name: "assets-test".to_string(),
        appname: "assets-test".to_string(),
        template: None,
        template_install_args: None,
        assertions: checks,
        plugins: None,
        deploy_args: None,
        skip_conditions: None,
        pre_build_hooks: None,
    };

    tc.run(controller).await.unwrap()
}
