use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use media::VideoCodec;

pub fn another_request() -> ViewerRequest {
    ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 60,
        }),
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
    }
}
