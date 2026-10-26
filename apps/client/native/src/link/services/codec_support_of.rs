use crate::link::DEFAULT_BIT_DEPTH;
use wire::{Codec, CodecSupport};

pub(crate) fn codec_support_of(id: i32, max_bit_depth: i32) -> Option<CodecSupport> {
    let codec = match id {
        1 => Codec::Avc,
        2 => Codec::Hevc,
        3 => Codec::Av1,
        _ => return None,
    };

    Some(CodecSupport {
        codec,
        max_bit_depth: u8::try_from(max_bit_depth).unwrap_or(DEFAULT_BIT_DEPTH),
    })
}
