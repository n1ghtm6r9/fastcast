use super::{frame_under, shared_nvenc_config};
use crate::web::{Admission, ViewerCursor};

#[test]
fn delta_frames_before_the_first_key_frame_do_not_announce_the_config() {
    let config = shared_nvenc_config();
    let mut cursor = ViewerCursor::starting_with(None);

    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Skip
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&config, true)),
        Admission::SendWithConfig(_)
    ));
}
