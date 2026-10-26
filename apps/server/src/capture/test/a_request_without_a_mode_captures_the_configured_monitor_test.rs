use super::default_settings;
use crate::capture::{CaptureTarget, open_screen};
use crate::hub::ViewerRequest;
use crate::settings::Settings;
use media::VideoCodec;

#[tokio::test]
async fn a_request_without_a_mode_captures_the_configured_monitor() {
    let settings = Settings {
        capture_output: 2,
        capture_fps: 144,
        ..default_settings()
    };
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: Vec::new(),
    };

    let screen = open_screen(&settings, &request).await;

    assert_eq!(
        screen.target,
        CaptureTarget {
            output: 2,
            fps: 144
        }
    );
    assert!(screen.display.is_none());
    assert_eq!(screen.request, request);
}
