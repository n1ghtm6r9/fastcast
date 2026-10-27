use crate::server::diagnose_run_of;
use crate::server::test::settings_named;
use crate::settings::Settings;
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn a_ticked_debug_box_names_the_run_by_the_start_time() {
    let settings = Settings {
        debug: true,
        ..settings_named("PC")
    };

    assert_eq!(
        diagnose_run_of(
            &settings,
            UNIX_EPOCH + Duration::from_millis(1_730_000_000_000)
        ),
        Some(1_730_000_000_000)
    );
}
