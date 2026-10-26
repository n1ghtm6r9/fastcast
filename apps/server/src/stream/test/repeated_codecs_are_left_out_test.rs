use crate::stream::codecs_of;
use media::VideoCodec;
use wire::{Codec, CodecSupport};

#[test]
fn repeated_codecs_are_left_out() {
    let listed = [
        CodecSupport {
            codec: Codec::Av1,
            max_bit_depth: 10,
        },
        CodecSupport {
            codec: Codec::Hevc,
            max_bit_depth: 10,
        },
        CodecSupport {
            codec: Codec::Hevc,
            max_bit_depth: 8,
        },
        CodecSupport {
            codec: Codec::Avc,
            max_bit_depth: 8,
        },
    ];

    assert_eq!(
        codecs_of(&listed),
        vec![VideoCodec::Av1, VideoCodec::Hevc, VideoCodec::Avc]
    );
}
