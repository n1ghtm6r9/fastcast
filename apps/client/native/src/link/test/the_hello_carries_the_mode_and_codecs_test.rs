use crate::link::hello_of;
use wire::{Codec, CodecSupport, Hello};

#[test]
fn the_hello_carries_the_mode_and_codecs() {
    let hello = hello_of(7, 3840, 2160, 120, &[2, 10, 1, 8], false);

    assert_eq!(
        hello,
        Hello {
            nonce: 7,
            width: 3840,
            height: 2160,
            fps: 120,
            codecs: vec![
                CodecSupport {
                    codec: Codec::Hevc,
                    max_bit_depth: 10,
                },
                CodecSupport {
                    codec: Codec::Avc,
                    max_bit_depth: 8,
                },
            ],
            noise: false,
        }
    );
}
