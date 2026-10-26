use super::hevc_sps_rbsp;

const TIER_FLAG_BYTE: usize = 3;
const TIER_FLAG_SHIFT: u8 = 5;

pub fn hevc_tier_flag(annex_b: &[u8]) -> u8 {
    (hevc_sps_rbsp(annex_b)[TIER_FLAG_BYTE] >> TIER_FLAG_SHIFT) & 1
}
