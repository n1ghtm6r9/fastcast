use crate::hub::Hub;

#[test]
fn a_requested_key_frame_is_taken_once() {
    let hub = Hub::new();

    hub.request_key_frame();

    assert!(hub.take_key_frame_request());
    assert!(!hub.take_key_frame_request());
}
