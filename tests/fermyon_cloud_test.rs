use e2e_testing::e2e_tests;
use e2e_testing::fc_controller::FermyonCloud;

#[cfg(test)]
mod fermyon_cloud_tests {
    use super::*;

    e2e_tests! {
        fermyon_cloud: FermyonCloud{},
    }
}
