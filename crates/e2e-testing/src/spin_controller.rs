use crate::controller::{AppInstance, Controller};
use crate::metadata_extractor::AppMetadata;
use crate::spin;
use crate::utils;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use std::process::{Command, Output};

pub struct SpinUp {}

#[async_trait]
impl Controller for SpinUp {
    fn name(&self) -> String {
        "spin-up".to_string()
    }

    fn login(&self) -> Result<()> {
        Ok(())
    }

    fn template_install(&self, args: Vec<&str>) -> Result<Output> {
        return spin::template_install(args);
    }

    fn new_app(&self, template_name: &str, app_name: &str) -> Result<Output> {
        return spin::new_app(template_name, app_name);
    }

    fn install_plugins(&self, plugins: Vec<&str>) -> Result<Output> {
        return spin::install_plugins(plugins);
    }

    fn build_app(&self, app_name: &str) -> Result<Output> {
        return spin::build_app(app_name);
    }

    async fn run_app(&self, app_name: &str) -> Result<AppInstance> {
        let appdir: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "..",
            "..",
            "tests",
            "testcases",
            app_name,
        ]
        .iter()
        .collect();

        let port = utils::get_random_port()?;
        let address = format!("127.0.0.1:{}", port);

        println!("before spin up");

        let mut spin_handle = Command::new("spin")
            .arg("up")
            .arg("--listen")
            .arg(&address)
            .env(
                "RUST_LOG",
                "spin=trace,spin_loader=trace,spin_core=trace,spin_http=trace",
            )
            .current_dir(appdir)
            .spawn()
            .with_context(|| format!("Unable to run spin up on {}", address))
            .unwrap();

        println!("after spin up");
        // ensure the server is accepting requests before continuing.
        utils::wait_tcp(&address, &mut spin_handle, "spin").await?;
        println!("after wait_tcp");

        Ok(AppInstance::new_with_process(
            AppMetadata {
                name: app_name.to_string(),
                base: format!("http://{}", address.to_string()),
                app_routes: vec![],
                version: "".to_string(),
            },
            Some(spin_handle),
        ))
    }
}
