use crate::spin_controller::{App, Controller};
use tokio::task;

pub struct TestCase {
    pub name: String,
    pub appname: String,
    pub template: Option<String>,
    pub assertions: fn(app: &App),
}

impl TestCase {
    pub async fn run(&self, controller: &dyn Controller) -> Result<(), String> {
        controller.name();
        match controller.template_install() {
            Err(error) => panic!("problem installing templates {:?}", error),
            _ => (),
        }

        match controller.new_app(&self.template.as_ref().unwrap(), &self.appname) {
            Err(error) => panic!("problem creating new app {:?}", error),
            _ => (),
        }

        match controller.build_app(&self.appname) {
            Err(error) => panic!("problem building new app {:?}", error),
            _ => (),
        }

        //TODO (rjindal): get child process and metadata here
        let app = match controller.deploy_app(&self.appname).await {
            Err(error) => panic!("problem building new app {:?}", error),
            Ok(app) => app,
        };

        let assert_fn = self.assertions;
        //test specific assertions
        task::spawn_blocking(move || {
            assert_fn(&app);
        });

        print!("from inside run");
        Ok(())
    }
}
