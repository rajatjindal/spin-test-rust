use crate::controller::{App, Controller};
use anyhow::{Context, Result};
use tokio::task;

pub struct TestCase {
    pub name: String,
    pub appname: String,
    pub template: Option<String>,
    pub assertions: fn(app: &App) -> Result<()>,
}

impl TestCase {
    pub async fn run(&self, controller: &dyn Controller) -> Result<()> {
        controller.name();
        controller
            .template_install()
            .context("installing templates")?;

        controller
            .new_app(&self.template.as_ref().unwrap(), &self.appname)
            .context("creating new app")?;

        controller.build_app(&self.appname).context("builing app")?;

        let app = controller
            .run_app(&self.appname)
            .await
            .context("deploying app")?;

        //test specific assertions
        let assert_fn = self.assertions;

        return task::spawn_blocking(move || {
            return assert_fn(&app);
        })
        .await
        .context("running testcase specific assertions")?;
    }
}
