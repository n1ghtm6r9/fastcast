use super::nal_units;

const HEVC_SPS: u8 = 33;

pub fn hevc_profile_idc(annex_b: &[u8]) -> u8 {
    let sps = nal_units(annex_b)
        .into_iter()
        .find(|unit| (unit[0] >> 1) & 0x3F == HEVC_SPS)
        .expect("an SPS NAL unit");

    sps[3] & 0x1F
}
