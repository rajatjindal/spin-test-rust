use crate::spin_controller::Controller;

pub struct TestCase {
    pub name: String,
    pub appname: String,
    pub template: Option<String>,
    pub metadata_extractor: Option<usize>,
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

        match controller.deploy_app(&self.appname) {
            // Err(error) => panic!("problem building new app {:?}", error),
            _ => (),
        }

        print!("from inside run");
        Ok(())
    }
}
