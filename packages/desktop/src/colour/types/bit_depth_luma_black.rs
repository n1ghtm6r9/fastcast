use crate::colour::BitDepth;

impl BitDepth {
    pub fn luma_black(self) -> f32 {
        match self {
            BitDepth::Eight => 16.0,
            BitDepth::Ten => 64.0,
        }
    }
}
