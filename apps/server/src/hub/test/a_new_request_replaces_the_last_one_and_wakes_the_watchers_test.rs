use crate::display::DisplayMode;
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;

#[tokio::test]
async fn a_new_request_replaces_the_last_one_and_wakes_the_watchers() {
    let hub = Hub::new();
    let mut watcher = hub.watch_requests();
    let request = ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 60,
        }),
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ten_bit_codecs: Vec::new(),
        noise: false,
    };

    assert!(hub.request(request.clone()));

    watcher.changed().await.expect("the hub is alive");

    assert_eq!(*watcher.borrow(), request);
}
