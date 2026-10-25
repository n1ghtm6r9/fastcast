use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use crate::web::{ViewerQuery, viewer_request_from};
use media::VideoCodec;

#[test]
fn a_full_query_asks_for_that_mode_and_those_codecs() {
    let query = ViewerQuery {
        width: Some("3840".into()),
        height: Some("2160".into()),
        fps: Some("60".into()),
        codecs: Some("hevc,avc".into()),
    };

    assert_eq!(
        viewer_request_from(&query),
        ViewerRequest {
            mode: Some(DisplayMode {
                width: 3840,
                height: 2160,
                refresh_hz: 60,
            }),
            codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        }
    );
}
