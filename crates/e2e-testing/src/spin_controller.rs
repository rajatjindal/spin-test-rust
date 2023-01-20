use crate::controller::{AppDetails, Controller};
use crate::metadata_extractor::Metadata;
use crate::utils;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::time::SystemTime;
use std::{
    fs,
    process::{Command, Output},
};

pub struct SpinUp {}

#[async_trait]
impl Controller for SpinUp {
    fn name(&self) -> String {
        "spin-up".to_string()
    }

    fn login(&self) -> Result<()> {
        Ok(())
    }

    fn template_install(&self) -> Result<Output> {
        return utils::run(
            vec![
                "spin",
                "templates",
                "install",
                "--git",
                "https://github.com/fermyon/spin",
            ],
            None,
            None,
        );
    }

    fn new_app(&self, template_name: &str, app_name: &str) -> Result<Output> {
        println!("{:?} new_app inside spin up", SystemTime::UNIX_EPOCH);
        match fs::remove_dir_all(app_name) {
            Err(_) => (),
            Ok(_) => (),
        }

        return utils::run(
            vec!["spin", "new", template_name, app_name, "--accept-defaults"],
            None,
            None,
        );
    }

    fn build_app(&self, app_name: &str) -> Result<Output> {
        println!("{:?} build_app inside spin up", SystemTime::UNIX_EPOCH);
        return utils::run(vec!["spin", "build"], Some(app_name), None);
    }

    async fn run_app(&self, app_name: &str) -> Result<AppDetails> {
        println!("{:?} deploy_app inside spin up", SystemTime::UNIX_EPOCH);

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
            .current_dir(app_name)
            .spawn()
            .with_context(|| format!("Unable to run spin up on {}", address))
            .unwrap();

        println!("after spin up");
        // ensure the server is accepting requests before continuing.
        utils::wait_tcp(&address, &mut spin_handle, "spin").await?;
        println!("after wait_tcp");

        Ok(AppDetails::new_with_process(
            Metadata {
                name: app_name.to_string(),
                base: format!("http://{}", address.to_string()),
                app_routes: vec![],
                version: "".to_string(),
            },
            Some(spin_handle),
        ))
    }
}
