use super::flv_header_bytes_declaring;
use crate::flv::FLV_HEADER_LENGTH;

pub fn flv_header_bytes() -> Vec<u8> {
    flv_header_bytes_declaring(FLV_HEADER_LENGTH as u32)
}
