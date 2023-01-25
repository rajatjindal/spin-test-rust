use crate::controller::{AppInstance, Controller};
use crate::utils;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tokio::task;

pub struct SkipCondition {
    pub env: String,
    pub reason: String,
}

impl SkipCondition {
    pub fn skip(&self, controller: &dyn Controller) -> bool {
        return controller.name() == self.env;
    }
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

        // evaluate the skip conditions specified in testcase config.
        if let Some(skip_conditions) = &self.skip_conditions {
            for skip_condition in skip_conditions {
                if skip_condition.skip(controller) {
                    return Ok(());
                }
            }
        }

        // install spin templates. If template_install_args is provided uses that, else
        // uses default spin repo
        let template_install_args = match &self.template_install_args {
            Some(args) => args.iter().map(|s| s as &str).collect(),
            None => vec!["--git", "https://github.com/fermyon/spin"],
        };

        controller
            .template_install(template_install_args)
            .context("installing templates")?;

        // install spin plugins if requested in testcase config
        if let Some(plugins) = &self.plugins {
            controller
                .install_plugins(plugins.iter().map(|s| s as &str).collect())
                .context("installing plugins")?;
        }

        let basedir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "..", "tests", "testcases"]
            .iter()
            .collect();
        let appdir = basedir.join(&self.appname);

        // cleanup existing dir for testcase project code. cleaned up only if testcase is a template based test
        if let Some(_) = &self.template {
            match fs::remove_dir_all(&appdir) {
                Err(_) => (),
                Ok(_) => (),
            }

            controller
                .new_app(&self.template.as_ref().unwrap(), &self.appname)
                .context("creating new app")?;
        }

        // run pre-build-steps. It is useful for running any steps required before running `spin build`.
        // e.g. for js/ts tests, we need to run `npm install` before running `spin build`
        if let Some(pre_build_hooks) = &self.pre_build_hooks {
            for pre_build_hook in pre_build_hooks {
                utils::run(
                    pre_build_hook.to_vec(),
                    Some(appdir.to_str().unwrap().to_string()),
                    None,
                )?;
            }
        }

        // run spin build
        controller.build_app(&self.appname).context("builing app")?;

        // run `spin up` (or `spin deploy` for cloud).
        // `AppInstance` has some basic info about the running app like base url, routes (only for cloud) etc.
        let app = controller
            .run_app(&self.appname)
            .await
            .context("deploying app")?;

        // run test specific assertions
        let assert_fn = self.assertions;

        return task::spawn_blocking(move || {
            return assert_fn(&app);
        })
        .await
        .context("running testcase specific assertions")?;
    }
}
