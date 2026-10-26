use super::{frame_under, shared_nvenc_config};
use crate::stream::{Admission, SessionCursor};
use std::sync::Arc;

#[test]
fn the_first_key_frame_announces_the_config_to_a_viewer_who_joined_without_one() {
    let config = shared_nvenc_config();
    let mut cursor = SessionCursor::starting_with(None);

    let Admission::SendWithConfig(announced) = cursor.admit(&frame_under(&config, true)) else {
        panic!("the config must be announced before the first key frame");
    };

    assert!(Arc::ptr_eq(&announced, &config));
    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Send
    ));
}
