use crate::hub::Hub;

#[test]
fn key_frame_requests_made_before_a_take_count_as_one() {
    let hub = Hub::new();

    hub.request_key_frame();
    hub.request_key_frame();

    assert!(hub.take_key_frame_request());
    assert!(!hub.take_key_frame_request());
}
