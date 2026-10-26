use super::{frame_under, shared_nvenc_config, shared_x264_config};
use crate::stream::{Admission, SessionCursor};

#[test]
fn a_delta_frame_under_a_config_the_viewer_was_not_told_about_is_skipped() {
    let first = shared_nvenc_config();
    let second = shared_x264_config();
    let mut cursor = SessionCursor::starting_with(Some(first.clone()));

    cursor.admit(&frame_under(&first, true));

    assert!(matches!(
        cursor.admit(&frame_under(&second, false)),
        Admission::Skip
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&second, true)),
        Admission::SendWithConfig(_)
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&second, false)),
        Admission::Send
    ));
}
