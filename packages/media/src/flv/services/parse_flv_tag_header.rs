use crate::flv::{FLV_TAG_HEADER_LENGTH, FlvTagHeader};

pub fn parse_flv_tag_header(bytes: &[u8; FLV_TAG_HEADER_LENGTH]) -> FlvTagHeader {
    FlvTagHeader {
        kind: bytes[0],
        data_size: u32::from_be_bytes([0, bytes[1], bytes[2], bytes[3]]),
        timestamp_ms: u32::from_be_bytes([bytes[7], bytes[4], bytes[5], bytes[6]]),
    }
}
