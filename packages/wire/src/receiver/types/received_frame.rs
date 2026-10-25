use crate::packet::FrameKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedFrame {
    pub number: u32,
    pub kind: FrameKind,
    pub keyframe: bool,
    pub payload: Vec<u8>,
}
