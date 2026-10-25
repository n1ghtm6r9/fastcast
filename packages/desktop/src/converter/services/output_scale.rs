use crate::colour::BitDepth;

pub fn output_scale(depth: BitDepth) -> f32 {
    match depth {
        BitDepth::Eight => 1.0 / 255.0,
        BitDepth::Ten => 64.0 / 65535.0,
    }
}
