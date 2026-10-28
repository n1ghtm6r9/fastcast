use crate::link::hello_of;
use wire::{Codec, CodecSupport};

#[test]
fn values_just_outside_their_range_are_replaced() {
    let hello = hello_of(7, 65_536, 65_535, -1, &[2, 255, 3, 0, 1, 256, 2, -1], false);

    assert_eq!(hello.width, 0);
    assert_eq!(hello.height, 65_535);
    assert_eq!(hello.fps, 0);
    assert_eq!(
        hello.codecs,
        vec![
            CodecSupport {
                codec: Codec::Hevc,
                max_bit_depth: 255,
            },
            CodecSupport {
                codec: Codec::Av1,
                max_bit_depth: 0,
            },
            CodecSupport {
                codec: Codec::Avc,
                max_bit_depth: 8,
            },
            CodecSupport {
                codec: Codec::Hevc,
                max_bit_depth: 8,
            },
        ]
    );
}
