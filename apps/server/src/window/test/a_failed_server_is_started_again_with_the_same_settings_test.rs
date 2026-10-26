use crate::server::{ServerState, ServerStatus};
use crate::window::test::{
    applied_settings, named_settings, press_button, saving_works, test_window, window_harness,
};

#[test]
fn a_failed_server_is_started_again_with_the_same_settings() {
    let failed = ServerStatus {
        state: ServerState::Failed {
            reason: "udp 0.0.0.0:3278 is busy".to_owned(),
        },
        ..ServerStatus::default()
    };
    let (window, probe) = test_window(named_settings("Office PC"), failed, saving_works);
    let mut harness = window_harness(window);

    press_button(&mut harness, "Запустить снова");

    assert_eq!(applied_settings(&probe), vec![named_settings("Office PC")]);
}
