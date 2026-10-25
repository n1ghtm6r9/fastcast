use crate::avc::{ChromaFormat, FrameCrop};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HevcSpsFields {
    pub chroma_array_type: ChromaFormat,
    pub width_in_luma_samples: u32,
    pub height_in_luma_samples: u32,
    pub conformance_window: FrameCrop,
}
