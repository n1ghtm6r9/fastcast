use crate::packet::CodecSupport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    pub nonce: u32,
    pub width: u16,
    pub height: u16,
    pub fps: u16,
    pub codecs: Vec<CodecSupport>,
    pub noise: bool,
}
