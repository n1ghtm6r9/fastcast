use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;

#[test]
fn the_same_request_again_is_not_a_change() {
    let hub = Hub::new();
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Avc],
        ten_bit_codecs: Vec::new(),
    };

    assert!(hub.request(request.clone()));

    let watcher = hub.watch_requests();

    assert!(!hub.request(request));
    assert!(!watcher.has_changed().expect("the hub is alive"));
}
