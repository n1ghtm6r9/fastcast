#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Welcome {
    pub nonce: u32,
    pub shard_size: u16,
}
