use crate::metadata_extractor::{extract_app_metadata_from_logs, Metadata};
use crate::utils;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::io::{BufReader, Read};
use std::process::Child;
use std::{
    fs,
    process::{Command, Output},
};

// use std::{
//     collections::HashMap,
//     ffi::OsStr,
//     // fs,
//     net::{Ipv4Addr, SocketAddrV4, TcpListener},
//     // path::Path,
//     process::{self, Child, Command, Output},
//     time::Duration,
// };
// use tokio::{net::TcpStream, time::sleep};

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
        print!("stopping app with id {}", self.process.id());
        match self.process.kill() {
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
        // let port = utils::get_random_port()?;
        let address = format!("127.0.0.1:{}", 4040);

        let mut spin_handle = Command::new("spin")
            .arg("up")
            .arg("--listen")
            .arg(&address)
            .env(
                "RUST_LOG",
                "spin=trace,spin_loader=trace,spin_core=trace,spin_http=trace",
            )
            .current_dir(app_name)
            // .stdout(Stdio::piped()) //this
            .spawn()
            .with_context(|| "executing Spin")?;

        print!("after spin up");
        // ensure the server is accepting requests before continuing.
        utils::wait_tcp(&address, &mut spin_handle, "spin").await?;
        print!("after wait_tcp");

        let mut f = BufReader::new(spin_handle.stdout.take().unwrap());
        let mut logs = String::new();

        print!("starting thread");

        match f.read_to_string(&mut logs) {
            Err(e) => panic!("failed to read from stdout {:?}", e),
            Ok(_) => (),
        };

        print!("after thread");

        let metadata = extract_app_metadata_from_logs(app_name, &logs);
        Ok(App {
            process: spin_handle,
            metadata: metadata,
        })
    }
}
