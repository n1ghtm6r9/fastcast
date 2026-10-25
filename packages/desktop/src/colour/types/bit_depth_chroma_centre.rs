use crate::colour::BitDepth;

impl BitDepth {
    pub fn chroma_centre(self) -> f32 {
        match self {
            BitDepth::Eight => 128.0,
            BitDepth::Ten => 512.0,
        }
    }
}
