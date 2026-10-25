#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameLayout {
    pub frame_size: u32,
    pub data_shards: usize,
    pub blocks: usize,
}
