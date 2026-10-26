use super::{frame_under, shared_nvenc_config};
use crate::stream::{Admission, SessionCursor};
use std::sync::Arc;

#[test]
fn a_resynced_cursor_announces_the_config_again_before_the_next_key_frame() {
    let config = shared_nvenc_config();
    let mut cursor = SessionCursor::starting_with(Some(config.clone()));

    cursor.admit(&frame_under(&config, true));
    cursor.resync();

    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Skip
    ));

    let Admission::SendWithConfig(announced) = cursor.admit(&frame_under(&config, true)) else {
        panic!("a resynced cursor must announce the config again before its key frame");
    };

    assert!(Arc::ptr_eq(&announced, &config));
}
