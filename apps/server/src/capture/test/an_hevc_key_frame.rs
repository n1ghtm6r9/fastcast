use super::{annex_b_of, hvcc_parameter_sets_of};
use crate::fixtures::nvenc_3840x2160_hvcc;

pub fn an_hevc_key_frame() -> Vec<u8> {
    let [vps, sps, pps] = hvcc_parameter_sets_of(&nvenc_3840x2160_hvcc());

    annex_b_of(&[&vps, &sps, &pps, &[0x26, 0x01, 0xAF, 0x08]])
}
