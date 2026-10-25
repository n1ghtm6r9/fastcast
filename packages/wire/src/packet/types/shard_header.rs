use crate::packet::FrameKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShardHeader {
    pub frame: u32,
    pub frame_kind: FrameKind,
    pub keyframe: bool,
    pub block: u16,
    pub blocks: u16,
    pub data_shards: u8,
    pub parity_shards: u8,
    pub shard: u8,
    pub frame_size: u32,
}
