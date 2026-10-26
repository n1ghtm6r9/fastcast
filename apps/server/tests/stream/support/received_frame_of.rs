use crate::support::VideoFrame;
use wire::ReceivedFrame;

pub fn received_frame_of(frame: &ReceivedFrame) -> VideoFrame {
    let wire = &frame.payload;

    VideoFrame {
        key: wire[0] & 1 == 1,
        timestamp: u32::from_le_bytes(wire[1..5].try_into().expect("the header has a timestamp")),
        data: wire[5..].to_vec(),
    }
}
