use super::{frame_under, shared_nvenc_config};
use crate::stream::{Admission, SessionCursor};

#[test]
fn a_key_frame_under_the_config_the_viewer_already_has_is_sent_as_it_is() {
    let config = shared_nvenc_config();
    let mut cursor = SessionCursor::starting_with(Some(config.clone()));

    assert!(matches!(
        cursor.admit(&frame_under(&config, true)),
        Admission::Send
    ));
    assert!(matches!(
        cursor.admit(&frame_under(&config, true)),
        Admission::Send
    ));
}
