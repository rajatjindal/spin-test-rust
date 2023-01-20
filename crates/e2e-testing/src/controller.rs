use crate::metadata_extractor::Metadata;
use anyhow::Result;
use async_trait::async_trait;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::Child;
use std::process::Output;

#[async_trait]
pub trait Controller {
    fn name(&self) -> String;
    fn login(&self) -> Result<()>;
    fn template_install(&self) -> Result<Output>;
    fn new_app(&self, template_name: &str, app_name: &str) -> Result<Output>;
    fn build_app(&self, app_name: &str) -> Result<Output>;
    async fn run_app(&self, app_name: &str) -> Result<App>;
}

pub struct App {
    pub metadata: Metadata,
    process: Option<Child>,
}

impl App {
    pub fn new(metadata: Metadata) -> App {
        App {
            metadata,
            process: None,
        }
    }

    pub fn new_with_process(metadata: Metadata, process: Option<Child>) -> App {
        App { metadata, process }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        match &self.process {
            None => (),
            Some(process) => {
                println!("stopping app with pid {}", process.id());
                let pid = Pid::from_raw(process.id() as i32);
                match kill(pid, Signal::SIGINT) {
                    Err(e) => panic!("error when stopping app with pid {}. {:?}", process.id(), e),
                    Ok(_) => (),
                }
            }
        }
    }
}
