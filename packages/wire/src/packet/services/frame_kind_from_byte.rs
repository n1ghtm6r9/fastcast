use crate::packet::FrameKind;

pub fn frame_kind_from_byte(byte: u8) -> Option<FrameKind> {
    match byte {
        0 => Some(FrameKind::Video),
        1 => Some(FrameKind::Config),
        _ => None,
    }
}
