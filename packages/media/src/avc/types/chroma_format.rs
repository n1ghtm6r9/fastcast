#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromaFormat {
    Monochrome,
    Yuv420,
    Yuv422,
    Yuv444,
}

impl ChromaFormat {
    pub fn from_idc(chroma_format_idc: u32) -> Option<Self> {
        match chroma_format_idc {
            0 => Some(Self::Monochrome),
            1 => Some(Self::Yuv420),
            2 => Some(Self::Yuv422),
            3 => Some(Self::Yuv444),
            _ => None,
        }
    }

    pub fn crop_unit(self) -> (u64, u64) {
        match self {
            Self::Monochrome | Self::Yuv444 => (1, 1),
            Self::Yuv420 => (2, 2),
            Self::Yuv422 => (2, 1),
        }
    }
}
