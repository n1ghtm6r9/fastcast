use crate::colour::BitDepth;
use crate::converter::{NoiseParameters, output_scale};

impl NoiseParameters {
    pub fn new(depth: BitDepth, frame: u32) -> NoiseParameters {
        NoiseParameters {
            frame,
            levels: depth.code_count(),
            output_scale: output_scale(depth),
            padding: 0,
        }
    }
}
