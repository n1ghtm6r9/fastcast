use crate::colour::BitDepth;

impl BitDepth {
    pub fn chroma_range(self) -> f32 {
        match self {
            BitDepth::Eight => 224.0,
            BitDepth::Ten => 896.0,
        }
    }
}
