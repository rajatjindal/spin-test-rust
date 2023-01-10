pub trait Controller {
    fn name(&self) -> String;
    // fn login() -> Result<(), String>;
    fn template_install(&self) -> Result<(), String>;
    // fn new_app() -> Result<(), String>;
    // fn build() -> Result<(), String>;
    // fn deploy() -> Result<(), String>;
    // fn stop() -> Result<(), String>;
}

pub struct SpinUp {}

impl Controller for SpinUp {
    fn name(&self) -> String {
        "spin-up".to_string()
    }

    // fn login() -> Result<(), String> {}

    fn template_install(&self) -> Result<(), String> {
        Ok(())
    }

    // fn new_app() -> Result<(), String> {}

    // fn build() -> Result<(), String> {}

    // fn deploy() -> Result<(), String> {}

    // fn stop() -> Result<(), String> {}
}
