use crate::capture::shown_at;
use std::time::Instant;

#[test]
fn a_picture_never_presented_has_no_screen_time() {
    assert_eq!(shown_at(0, Instant::now()), None);
}
