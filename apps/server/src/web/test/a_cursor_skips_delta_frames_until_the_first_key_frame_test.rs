use super::{frame_under, shared_nvenc_config};
use crate::web::{Admission, ViewerCursor};

#[test]
fn a_cursor_skips_delta_frames_until_the_first_key_frame() {
    let config = shared_nvenc_config();
    let mut cursor = ViewerCursor::starting_with(Some(config.clone()));

    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Skip
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Skip
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&config, true)),
        Admission::Send
    ));
}
