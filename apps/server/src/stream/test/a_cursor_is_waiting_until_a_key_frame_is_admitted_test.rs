use super::{frame_under, shared_nvenc_config};
use crate::stream::SessionCursor;

#[test]
fn a_cursor_is_waiting_until_a_key_frame_is_admitted() {
    let config = shared_nvenc_config();
    let mut cursor = SessionCursor::starting_with(Some(config.clone()));

    assert!(cursor.is_waiting());

    cursor.admit(&frame_under(&config, true));

    assert!(!cursor.is_waiting());

    cursor.lagged();

    assert!(cursor.is_waiting());

    cursor.admit(&frame_under(&config, true));
    cursor.resync();

    assert!(cursor.is_waiting());
}
