#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Feedback {
    pub highest_frame: u32,
    pub received_packets: u32,
    pub highest_sequence: u32,
    pub lost_frames: u32,
    pub recovered_shards: u32,
    pub keyframe_needed: bool,
}
