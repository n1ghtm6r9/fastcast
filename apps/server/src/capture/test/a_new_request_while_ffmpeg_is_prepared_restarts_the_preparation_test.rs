use super::{FakeBackend, FakePreparation, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::display::DisplayMode;
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn a_new_request_while_ffmpeg_is_prepared_restarts_the_preparation() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(
        vec![FakePreparation::Takes {
            delay: Duration::from_secs(60),
        }],
        vec![],
    );
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
        noise: false,
    };

    settle().await;
    hub.request(request.clone());
    settle().await;

    assert_eq!(backend.prepares(), 2);
    assert_eq!(backend.opened(), vec![request]);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
