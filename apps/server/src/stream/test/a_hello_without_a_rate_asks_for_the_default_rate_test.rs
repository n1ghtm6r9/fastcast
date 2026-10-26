use super::hello_of;
use crate::display::DisplayMode;
use crate::stream::viewer_request_of;

#[test]
fn a_hello_without_a_rate_asks_for_the_default_rate() {
    let hello = hello_of(1920, 1080, 0, Vec::new());

    assert_eq!(
        viewer_request_of(&hello, 60).mode,
        Some(DisplayMode {
            width: 1920,
            height: 1080,
            refresh_hz: 60,
        })
    );
}
