use super::hello_of;
use crate::stream::viewer_request_of;

#[test]
fn a_mode_outside_the_supported_range_asks_for_no_mode() {
    let too_small = hello_of(100, 100, 60, Vec::new());
    let too_fast = hello_of(1920, 1080, 241, Vec::new());

    assert_eq!(viewer_request_of(&too_small, 60).mode, None);
    assert_eq!(viewer_request_of(&too_fast, 60).mode, None);
}
