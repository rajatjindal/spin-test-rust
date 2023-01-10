use crate::spin_controller::Controller;

pub struct TestCase {
    pub name: String,
    pub appname: String,
    pub template: Option<String>,
    pub metadata_extractor: Option<usize>,
}

impl TestCase {
    pub fn run(&self, controller: &dyn Controller) -> Result<(), String> {
        controller.name();
        print!("from inside run");
        Ok(())
    }
}
