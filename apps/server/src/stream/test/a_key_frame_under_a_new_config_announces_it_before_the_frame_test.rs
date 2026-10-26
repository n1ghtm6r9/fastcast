use super::{frame_under, shared_nvenc_config, shared_x264_config};
use crate::stream::{Admission, SessionCursor};
use std::sync::Arc;

#[test]
fn a_key_frame_under_a_new_config_announces_it_before_the_frame() {
    let first = shared_nvenc_config();
    let second = shared_x264_config();
    let mut cursor = SessionCursor::starting_with(Some(first.clone()));

    cursor.admit(&frame_under(&first, true));

    let Admission::SendWithConfig(announced) = cursor.admit(&frame_under(&second, true)) else {
        panic!("the new config must be announced before its first key frame");
    };

    assert!(Arc::ptr_eq(&announced, &second));
    assert!(matches!(
        cursor.admit(&frame_under(&second, false)),
        Admission::Send
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&second, true)),
        Admission::Send
    ));
}
