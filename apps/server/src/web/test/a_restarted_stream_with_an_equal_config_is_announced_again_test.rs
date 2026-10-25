use super::{frame_under, shared_nvenc_config};
use crate::web::{Admission, ViewerCursor};

#[test]
fn a_restarted_stream_with_an_equal_config_is_announced_again() {
    let before = shared_nvenc_config();
    let after = shared_nvenc_config();
    let mut cursor = ViewerCursor::starting_with(Some(before.clone()));

    cursor.admit(&frame_under(&before, true));

    assert_eq!(*before, *after);
    assert!(matches!(
        cursor.admit(&frame_under(&after, true)),
        Admission::SendWithConfig(_)
    ));
}
