use crate::controller::{AppInstance, Controller};
use crate::metadata_extractor::AppMetadata;
use crate::utils;
use anyhow::{Context, Result};
use async_trait::async_trait;
use lockfile::Lockfile;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Duration;
use waitfor::wait_for;

pub struct SpinUp {}

#[async_trait]
impl Controller for SpinUp {
    fn name(&self) -> String {
        "spin-up".to_string()
    }

    fn login(&self) -> Result<()> {
        Ok(())
    }

    fn template_install(&self, mut args: Vec<&str>) -> Result<Output> {
        let mut cmd = vec!["spin", "templates", "install"];
        cmd.append(&mut args);
        return utils::run(cmd, None, None);
    }

    fn new_app(&self, template_name: &str, app_name: &str) -> Result<Output> {
        let basedir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "..", "tests", "testcases"]
            .iter()
            .collect();

        return utils::run(
            vec!["spin", "new", template_name, app_name, "--accept-defaults"],
            basedir.to_str(),
            None,
        );
    }

    fn install_plugins(&self, plugins: Vec<&str>) -> Result<Output> {
        wait_for::<_, _, ()>(Duration::from_secs(30), Duration::from_secs(1), || {
            if Path::new("/tmp/installing-plugins.lock").exists() {
                return Ok(None);
            } else {
                Ok(Some("install plugins not running"))
            }
        })
        .unwrap();

        let lockfile = Lockfile::create("/tmp/installing-plugins.lock").unwrap();
        let mut output = utils::run(vec!["spin", "plugin", "update"], None, None)?;

        for plugin in plugins {
            output = utils::run(
                vec!["spin", "plugin", "install", plugin, "--yes"],
                None,
                None,
            )?;
        }

        lockfile.release()?;
        Ok(output)
    }

    fn build_app(&self, app_name: &str) -> Result<Output> {
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

        println!("appdir is {:?}", appdir.to_str());
        return utils::run(vec!["spin", "build"], appdir.to_str(), None);
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
