use super::hevc_sps_rbsp;

const LEVEL_IDC_BYTE: usize = 14;

pub fn hevc_level_idc(annex_b: &[u8]) -> u8 {
    hevc_sps_rbsp(annex_b)[LEVEL_IDC_BYTE]
}
