use e2e_testing::e2e_tests;
use e2e_testing::spin_controller::SpinUp;

#[cfg(test)]
mod spinup_tests {
    use super::*;

    e2e_tests! {
        spin_up: SpinUp{},
    }
}
