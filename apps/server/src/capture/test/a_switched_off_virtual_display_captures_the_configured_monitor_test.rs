use super::default_settings;
use crate::capture::{CaptureTarget, open_screen};
use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use crate::settings::Settings;
use media::VideoCodec;

#[tokio::test]
async fn a_switched_off_virtual_display_captures_the_configured_monitor() {
    let settings = Settings {
        virtual_display: false,
        ..default_settings()
    };
    let request = ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 60,
        }),
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ten_bit_codecs: Vec::new(),
    };

    let screen = open_screen(&settings, &request, |_| Some(60)).await;

    assert_eq!(screen.target, CaptureTarget { output: 0, fps: 60 });
    assert!(screen.display.is_none());
}
