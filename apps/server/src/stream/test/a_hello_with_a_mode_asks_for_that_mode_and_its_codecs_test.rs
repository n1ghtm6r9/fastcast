use super::hello_of;
use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use crate::stream::viewer_request_of;
use media::VideoCodec;
use wire::{Codec, CodecSupport};

#[test]
fn a_hello_with_a_mode_asks_for_that_mode_and_its_codecs() {
    let hello = hello_of(
        3840,
        2160,
        120,
        vec![
            CodecSupport {
                codec: Codec::Hevc,
                max_bit_depth: 10,
            },
            CodecSupport {
                codec: Codec::Avc,
                max_bit_depth: 8,
            },
        ],
    );

    assert_eq!(
        viewer_request_of(&hello, 60),
        ViewerRequest {
            mode: Some(DisplayMode {
                width: 3840,
                height: 2160,
                refresh_hz: 120,
            }),
            codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
            ten_bit_codecs: vec![VideoCodec::Hevc],
            noise: false,
        }
    );
}
