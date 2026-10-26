use super::hello_of;
use crate::stream::viewer_request_of;

#[test]
fn a_hello_without_a_size_asks_for_the_configured_monitor() {
    let hello = hello_of(0, 0, 60, Vec::new());

    assert_eq!(viewer_request_of(&hello, 60).mode, None);
}
