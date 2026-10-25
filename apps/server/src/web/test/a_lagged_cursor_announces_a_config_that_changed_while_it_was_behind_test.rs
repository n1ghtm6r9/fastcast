use super::{frame_under, shared_nvenc_config, shared_x264_config};
use crate::web::{Admission, ViewerCursor};

#[test]
fn a_lagged_cursor_announces_a_config_that_changed_while_it_was_behind() {
    let first = shared_nvenc_config();
    let second = shared_x264_config();
    let mut cursor = ViewerCursor::starting_with(Some(first.clone()));

    cursor.admit(&frame_under(&first, true));
    cursor.lagged();

    assert!(matches!(
        cursor.admit(&frame_under(&second, false)),
        Admission::Skip
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&second, true)),
        Admission::SendWithConfig(_)
    ));
}
