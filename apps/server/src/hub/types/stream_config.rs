use bytes::Bytes;

#[derive(Debug, PartialEq, Eq)]
pub struct StreamConfig {
    pub record: Bytes,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub json: String,
}
