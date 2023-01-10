use crate::utils;
use anyhow::{Context, Result};
use async_trait::async_trait;
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
    async fn deploy_app(&self, app_name: &str) -> Result<()>;

    // fn stop() -> Result<(), String>;
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

    async fn deploy_app(&self, app_name: &str) -> Result<()> {
        let port = utils::get_random_port()?;
        let address = format!("127.0.0.1:{}", port);

        let mut spin_handle = Command::new("spin")
            .arg("up")
            .env(
                "RUST_LOG",
                "spin=trace,spin_loader=trace,spin_core=trace,spin_http=trace",
            )
            .current_dir(app_name)
            .spawn()
            .with_context(|| "executing Spin")?;

        // ensure the server is accepting requests before continuing.
        utils::wait_tcp(&address, &mut spin_handle, "spin").await?;

        //TODO (rjindal): how to do clean exit for this process
        Ok(())
    }
}
