use crate::hub::TEN_BIT_DEPTH;
use desktop::BitDepth;

pub fn converter_depth_of(bits: u8) -> BitDepth {
    if bits == TEN_BIT_DEPTH {
        BitDepth::Ten
    } else {
        BitDepth::Eight
    }
}
