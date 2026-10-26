use super::{FakeBackend, settle};
use crate::capture::supervise_capture;
use crate::display::DisplayMode;
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn a_request_for_other_codecs_in_the_same_mode_keeps_the_screen() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let _viewer = hub.join();
    let ten_bit = ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 120,
        }),
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ten_bit_codecs: vec![VideoCodec::Hevc],
    };
    let eight_bit = ViewerRequest {
        ten_bit_codecs: Vec::new(),
        ..ten_bit.clone()
    };

    settle().await;
    hub.request(ten_bit.clone());
    settle().await;
    hub.request(eight_bit);
    settle().await;

    assert_eq!(backend.starts(), 3);
    assert_eq!(backend.kills(), 2);
    assert_eq!(backend.opened(), vec![ViewerRequest::default(), ten_bit]);

    supervisor.abort();
}
