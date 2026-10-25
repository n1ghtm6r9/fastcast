use crate::packet::ShardHeader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shard<'a> {
    pub header: ShardHeader,
    pub data: &'a [u8],
}
