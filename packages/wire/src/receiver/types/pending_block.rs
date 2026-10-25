#[derive(Debug)]
pub struct PendingBlock {
    pub(crate) data_shards: usize,
    pub(crate) parity_shards: usize,
    pub(crate) present: Vec<bool>,
    pub(crate) present_count: usize,
    pub(crate) data_present: usize,
    pub(crate) bytes: Vec<u8>,
}
