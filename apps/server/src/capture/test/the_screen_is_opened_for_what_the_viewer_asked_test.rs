use super::{FakeBackend, settle};
use crate::capture::supervise_capture;
use crate::display::DisplayMode;
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn the_screen_is_opened_for_what_the_viewer_asked() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let request = ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 60,
        }),
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: Vec::new(),
    };

    hub.request(request.clone());
    let _viewer = hub.join();
    settle().await;

    assert_eq!(backend.opened(), vec![request]);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
