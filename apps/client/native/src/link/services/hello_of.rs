use crate::link::{codec_support_of, u16_or_zero};
use wire::Hello;

pub fn hello_of(nonce: u32, width: i32, height: i32, fps: i32, codecs: &[i32]) -> Hello {
    Hello {
        nonce,
        width: u16_or_zero(width),
        height: u16_or_zero(height),
        fps: u16_or_zero(fps),
        codecs: codecs
            .as_chunks::<2>()
            .0
            .iter()
            .filter_map(|[id, max_bit_depth]| codec_support_of(*id, *max_bit_depth))
            .collect(),
    }
}
