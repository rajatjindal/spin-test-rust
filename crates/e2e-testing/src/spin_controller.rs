use crate::metadata_extractor::{extract_app_metadata_from_logs, Metadata};
use crate::utils;
use anyhow::{Context, Result};
use async_trait::async_trait;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::Child;
use std::{
    fs,
    process::{Command, Output},
};

#[async_trait]
pub trait Controller {
    fn name(&self) -> String;
    fn login(&self) -> Result<()>;
    fn template_install(&self) -> Result<Output>;
    fn new_app(&self, template_name: &str, app_name: &str) -> Result<Output>;
    fn build_app(&self, app_name: &str) -> Result<Output>;
    async fn deploy_app(&self, app_name: &str) -> Result<App>;

    // fn stop() -> Result<(), String>;
}

pub struct App {
    process: Child,
    pub metadata: Metadata,
}

impl Drop for App {
    fn drop(&mut self) {
        println!("stopping app with id {}", self.process.id());
        let pid = Pid::from_raw(self.process.id() as i32);
        match kill(pid, Signal::SIGINT) {
            Err(e) => panic!(
                "error when stopping app with id {}. {:?}",
                self.process.id(),
                e
            ),
            Ok(_) => (),
        }
    }
}

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
        return utils::run(vec!["spin", "build"], Some(app_name), None);
    }

    async fn deploy_app(&self, app_name: &str) -> Result<App> {
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

        Ok(App {
            process: spin_handle,
            metadata: Metadata {
                name: app_name.to_string(),
                base: address.to_string(),
                app_routes: vec![],
                version: "".to_string(),
            },
        })
    }
}
