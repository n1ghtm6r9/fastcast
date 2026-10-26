#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoFrame {
    pub key: bool,
    pub timestamp: u32,
    pub data: Vec<u8>,
}
