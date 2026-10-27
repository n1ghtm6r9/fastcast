use crate::server::diagnose_run_of;
use crate::server::test::settings_named;
use std::time::SystemTime;

#[test]
fn an_unticked_debug_box_asks_for_no_diagnosis() {
    assert_eq!(
        diagnose_run_of(&settings_named("PC"), SystemTime::now()),
        None
    );
}
