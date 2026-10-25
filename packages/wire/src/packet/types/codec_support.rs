use crate::packet::Codec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodecSupport {
    pub codec: Codec,
    pub max_bit_depth: u8,
}
