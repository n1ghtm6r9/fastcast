use super::hello_of;
use crate::stream::viewer_request_of;
use media::VideoCodec;
use wire::{Codec, CodecSupport};

#[test]
fn a_codec_offered_with_eight_bits_is_no_ten_bit_codec() {
    let hello = hello_of(
        3840,
        2160,
        120,
        vec![
            CodecSupport {
                codec: Codec::Hevc,
                max_bit_depth: 8,
            },
            CodecSupport {
                codec: Codec::Avc,
                max_bit_depth: 10,
            },
            CodecSupport {
                codec: Codec::Av1,
                max_bit_depth: 10,
            },
        ],
    );

    assert_eq!(
        viewer_request_of(&hello, 60).ten_bit_codecs,
        vec![VideoCodec::Avc, VideoCodec::Av1]
    );
}
