use crate::packet::FrameKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutgoingFrame<'a> {
    pub kind: FrameKind,
    pub keyframe: bool,
    pub payload: &'a [u8],
}
