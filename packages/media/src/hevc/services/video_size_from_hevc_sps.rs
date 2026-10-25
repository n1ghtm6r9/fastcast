use crate::avc::{VideoSize, remove_emulation_prevention};
use crate::hevc::{
    HEVC_NAL_HEADER_LENGTH, HEVC_SPS_NAL_UNIT_TYPE, hevc_nal_unit_type, parse_hevc_sps_fields,
    video_size_from_hevc_sps_fields,
};

pub fn video_size_from_hevc_sps(sps_nal_unit: &[u8]) -> Option<VideoSize> {
    let (header, payload) = sps_nal_unit.split_at_checked(HEVC_NAL_HEADER_LENGTH)?;

    if hevc_nal_unit_type(header[0]) != HEVC_SPS_NAL_UNIT_TYPE {
        return None;
    }

    let rbsp = remove_emulation_prevention(payload);
    let fields = parse_hevc_sps_fields(&rbsp)?;

    video_size_from_hevc_sps_fields(&fields)
}
