use crate::metadata_extractor::AppMetadata;
use anyhow::Result;
use async_trait::async_trait;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::Output;

#[async_trait]
pub trait Controller {
    fn name(&self) -> String;
    fn login(&self) -> Result<()>;
    fn template_install(&self, args: Vec<&str>) -> Result<Output>;
    fn new_app(&self, template_name: &str, app_name: &str) -> Result<Output>;
    fn build_app(&self, app_name: &str) -> Result<Output>;
    fn install_plugins(&self, plugins: Vec<&str>) -> Result<Output>;
    async fn run_app(&self, app_name: &str) -> Result<AppInstance>;
}

pub struct AppInstance {
    pub metadata: AppMetadata,
    process: Option<tokio::process::Child>,
}

impl AppInstance {
    pub fn new(metadata: AppMetadata) -> AppInstance {
        AppInstance {
            metadata,
            process: None,
        }
    }

    pub fn new_with_process(
        metadata: AppMetadata,
        process: Option<tokio::process::Child>,
    ) -> AppInstance {
        AppInstance { metadata, process }
    }
}

impl Drop for AppInstance {
    fn drop(&mut self) {
        match &self.process {
            None => (),
            Some(process) => {
                let pid = process.id().unwrap();
                println!("stopping app with pid {}", pid);
                let pid = Pid::from_raw(pid as i32);
                match kill(pid, Signal::SIGINT) {
                    Err(e) => panic!("error when stopping app with pid {}. {:?}", pid, e),
                    Ok(_) => (),
                }
            }
        }
    }
}
