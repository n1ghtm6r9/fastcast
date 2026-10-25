use crate::hub::{KEY_FRAME_FLAG, WIRE_HEADER_LENGTH};
use bytes::{BufMut, Bytes, BytesMut};

pub fn encode_wire_frame(key: bool, timestamp_ms: u32, data: &[u8]) -> Bytes {
    let mut wire = BytesMut::with_capacity(WIRE_HEADER_LENGTH + data.len());

    wire.put_u8(if key { KEY_FRAME_FLAG } else { 0 });
    wire.put_u32_le(timestamp_ms);
    wire.extend_from_slice(data);

    wire.freeze()
}
