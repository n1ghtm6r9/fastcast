use crate::hevc::{HEVC_NAL_UNIT_TYPE_MASK, HEVC_NAL_UNIT_TYPE_SHIFT};

pub fn hevc_nal_unit_type(first_header_byte: u8) -> u8 {
    (first_header_byte & HEVC_NAL_UNIT_TYPE_MASK) >> HEVC_NAL_UNIT_TYPE_SHIFT
}
