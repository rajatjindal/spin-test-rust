use crate::controller::{AppInstance, Controller};
use crate::utils;
use anyhow::{Context, Result};
use tokio::task;

pub struct SkipCondition {
    pub env: String,
    pub reason: String,
}

pub struct TestCase {
    pub name: String,
    pub appname: String,
    pub template: Option<String>,
    pub template_install_args: Option<Vec<String>>,
    pub plugins: Option<Vec<String>>,
    pub skip_conditions: Option<Vec<SkipCondition>>,
    pub deploy_args: Option<Vec<String>>,
    pub pre_build_hooks: Option<Vec<Vec<String>>>,
    pub assertions: fn(app: &AppInstance) -> Result<()>,
}

impl TestCase {
    pub async fn run(&self, controller: &dyn Controller) -> Result<()> {
        controller.name();

        let template_install_args = match &self.template_install_args {
            Some(args) => args.iter().map(|s| s as &str).collect(),
            None => vec!["--git", "https://github.com/fermyon/spin"],
        };

        controller
            .template_install(template_install_args)
            .context("installing templates")?;

        if let Some(plugins) = &self.plugins {
            controller
                .install_plugins(plugins.iter().map(|s| s as &str).collect())
                .context("installing plugins")?;
        }

        controller
            .new_app(&self.template.as_ref().unwrap(), &self.appname)
            .context("creating new app")?;

        if let Some(pre_build_hooks) = &self.pre_build_hooks {
            for pre_build_hook in pre_build_hooks {
                utils::run(
                    pre_build_hook.to_vec(),
                    Some(self.appname.to_string()),
                    None,
                )?;
            }
        }

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
