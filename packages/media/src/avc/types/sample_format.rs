#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleFormat {
    pub chroma_format_idc: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
}
