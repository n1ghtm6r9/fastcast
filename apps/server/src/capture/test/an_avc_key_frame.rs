use super::{annex_b_of, avcc_parameter_sets_of};
use crate::fixtures::nvenc_2560x1440_avcc;

pub fn an_avc_key_frame() -> Vec<u8> {
    let [sps, pps] = avcc_parameter_sets_of(&nvenc_2560x1440_avcc());

    annex_b_of(&[&sps, &pps, &[0x65, 0x88, 0x84, 0x21]])
}
