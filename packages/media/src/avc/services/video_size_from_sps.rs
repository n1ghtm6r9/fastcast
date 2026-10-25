use crate::avc::{
    NAL_UNIT_TYPE_MASK, SPS_NAL_UNIT_TYPE, VideoSize, parse_sps_fields,
    remove_emulation_prevention, video_size_from_sps_fields,
};

pub fn video_size_from_sps(sps_nal_unit: &[u8]) -> Option<VideoSize> {
    let (header, payload) = sps_nal_unit.split_first()?;

    if header & NAL_UNIT_TYPE_MASK != SPS_NAL_UNIT_TYPE {
        return None;
    }

    let rbsp = remove_emulation_prevention(payload);
    let fields = parse_sps_fields(&rbsp)?;

    video_size_from_sps_fields(&fields)
}
