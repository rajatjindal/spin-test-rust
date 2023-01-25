use crate::controller::{AppInstance, Controller};
use crate::metadata_extractor::extract_app_metadata_from_logs;
use crate::utils;
use anyhow::Result;
use async_trait::async_trait;
use lockfile::Lockfile;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;
use std::time::Duration;
use waitfor::wait_for;

pub struct FermyonCloud {}

#[async_trait]
impl Controller for FermyonCloud {
    fn name(&self) -> String {
        "fermyon-cloud".to_string()
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
        let basedir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "testcases"]
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
        let appdir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "testcases", app_name]
            .iter()
            .collect();

        return utils::run(vec!["spin", "build"], appdir.to_str(), None);
    }

    async fn run_app(&self, app_name: &str) -> Result<AppInstance> {
        let appdir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "testcases", app_name]
            .iter()
            .collect();

        match utils::run(vec!["spin", "deploy"], appdir.to_str(), None) {
            Err(error) => panic!("problem deploying app {:?}", error),
            Ok(result) => {
                let logs = match str::from_utf8(&result.stdout) {
                    Ok(logs) => logs,
                    Err(error) => panic!("problem fetching deploy logs for app {:?}", error),
                };

                let metadata = extract_app_metadata_from_logs(app_name, logs);
                return Ok(AppInstance::new(metadata));
            }
        };
    }
}
