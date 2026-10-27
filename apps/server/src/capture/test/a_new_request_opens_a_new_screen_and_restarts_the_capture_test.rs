use super::{FakeBackend, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::display::DisplayMode;
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn a_new_request_opens_a_new_screen_and_restarts_the_capture() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let _viewer = hub.join();
    let request = ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 60,
        }),
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ten_bit_codecs: Vec::new(),
    };

    settle().await;
    hub.request(request.clone());
    settle().await;

    assert_eq!(backend.starts(), 2);
    assert_eq!(backend.kills(), 1);
    assert_eq!(backend.opened(), vec![ViewerRequest::default(), request]);

    supervisor.abort();
}
