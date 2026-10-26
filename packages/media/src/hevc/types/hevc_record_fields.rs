use crate::avc::SampleFormat;
use crate::hevc::HEVC_PROFILE_TIER_LEVEL_LENGTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HevcRecordFields {
    pub general_profile_tier_level: [u8; HEVC_PROFILE_TIER_LEVEL_LENGTH],
    pub max_sub_layers_minus1: u8,
    pub temporal_id_nesting: bool,
    pub sample_format: SampleFormat,
}
