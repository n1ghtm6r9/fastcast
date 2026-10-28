use crate::colour::BitDepth;

impl BitDepth {
    pub fn code_count(self) -> u32 {
        match self {
            BitDepth::Eight => 256,
            BitDepth::Ten => 1024,
        }
    }
}
