use crate::avc::{ChromaFormat, FrameCrop};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpsFields {
    pub chroma_array_type: ChromaFormat,
    pub width_in_macroblocks: u32,
    pub height_in_map_units: u32,
    pub frame_mbs_only: bool,
    pub crop: FrameCrop,
}
