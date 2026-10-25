use crate::colour::BitDepth;

impl BitDepth {
    pub fn luma_range(self) -> f32 {
        match self {
            BitDepth::Eight => 219.0,
            BitDepth::Ten => 876.0,
        }
    }
}
