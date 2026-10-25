#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlvTagHeader {
    pub kind: u8,
    pub data_size: u32,
    pub timestamp_ms: u32,
}
