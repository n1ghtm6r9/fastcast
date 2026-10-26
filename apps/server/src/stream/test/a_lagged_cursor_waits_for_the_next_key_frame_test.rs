use super::{frame_under, shared_nvenc_config};
use crate::stream::{Admission, SessionCursor};

#[test]
fn a_lagged_cursor_waits_for_the_next_key_frame() {
    let config = shared_nvenc_config();
    let mut cursor = SessionCursor::starting_with(Some(config.clone()));

    cursor.admit(&frame_under(&config, true));

    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Send
    ));

    cursor.lagged();

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
    assert!(matches!(
        cursor.admit(&frame_under(&config, false)),
        Admission::Send
    ));
}
