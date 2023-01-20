use crate::controller::{App, Controller};
use crate::metadata_extractor::extract_app_metadata_from_logs;
use crate::utils;
use anyhow::Result;
use async_trait::async_trait;
use std::str;
use std::time::SystemTime;
use std::{fs, process::Output};

pub struct FermyonCloud {}

#[async_trait]
impl Controller for FermyonCloud {
    fn name(&self) -> String {
        "fermyon-cloud".to_string()
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
        println!("{:?} new_app inside fc", SystemTime::UNIX_EPOCH);

        match fs::remove_dir_all(app_name) {
            Ok(()) => (),
            Err(error) => panic!("problem cleaning up dir for new app {:?}", error),
        }

        return utils::run(
            vec!["spin", "new", template_name, app_name, "--accept-defaults"],
            None,
            None,
        );
    }

    fn build_app(&self, app_name: &str) -> Result<Output> {
        println!("{:?} build app inside fc", SystemTime::UNIX_EPOCH);
        return utils::run(vec!["spin", "build"], Some(app_name), None);
    }

    async fn deploy_app(&self, app_name: &str) -> Result<App> {
        println!("{:?} deploy_app inside fc", SystemTime::UNIX_EPOCH);

        match utils::run(vec!["spin", "deploy"], Some(app_name), None) {
            Err(error) => panic!("problem deploying app {:?}", error),
            Ok(result) => {
                let logs = match str::from_utf8(&result.stdout) {
                    Ok(logs) => logs,
                    Err(error) => panic!("problem fetching deploy logs for app {:?}", error),
                };

                let metadata = extract_app_metadata_from_logs(app_name, logs);
                return Ok(App::new(metadata));
            }
        };
    }
}
