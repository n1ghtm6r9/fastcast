use crate::hub::Frame;

pub fn frame_timestamp(frame: &Frame) -> u32 {
    let bytes: [u8; 4] = frame.wire[1..5]
        .try_into()
        .expect("a wire frame has a four byte timestamp");

    u32::from_le_bytes(bytes)
}
