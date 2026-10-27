use crate::server::diagnose_run_of;
use crate::server::test::settings_named;
use crate::settings::Settings;
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn a_clock_before_1970_still_names_a_diagnose_run() {
    let settings = Settings {
        debug: true,
        ..settings_named("PC")
    };

    assert_eq!(
        diagnose_run_of(&settings, UNIX_EPOCH - Duration::from_secs(1)),
        Some(1)
    );
}
