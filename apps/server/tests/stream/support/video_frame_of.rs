use crate::support::VideoFrame;

pub fn video_frame_of(wire: &[u8]) -> VideoFrame {
    VideoFrame {
        key: wire[0] & 1 == 1,
        timestamp: u32::from_le_bytes(wire[1..5].try_into().expect("the header has a timestamp")),
        data: wire[5..].to_vec(),
    }
}
