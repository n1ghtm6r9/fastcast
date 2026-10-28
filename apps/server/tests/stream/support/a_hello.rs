use crate::support::next_nonce;
use wire::{Codec, CodecSupport, Hello};

pub fn a_hello(width: u16, height: u16, fps: u16, codecs: &[Codec]) -> Hello {
    Hello {
        nonce: next_nonce(),
        width,
        height,
        fps,
        codecs: codecs
            .iter()
            .map(|&codec| CodecSupport {
                codec,
                max_bit_depth: 8,
            })
            .collect(),
        noise: false,
    }
}
