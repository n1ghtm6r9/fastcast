use crate::hub::Hub;

#[test]
fn no_key_frame_is_requested_before_anyone_asks() {
    let hub = Hub::new();

    assert!(!hub.take_key_frame_request());
}
