use super::{hvcc_parameter_sets, real_hvcc_record_named};
use crate::hevc::hvcc_from_parameter_sets;

#[test]
fn a_parameter_set_longer_than_a_u16_length_gives_no_record() {
    let (vps, sps, _) = hvcc_parameter_sets(&real_hvcc_record_named("nvenc_main_1920x1080").hvcc);
    let pps = vec![0x44; usize::from(u16::MAX) + 1];

    assert_eq!(hvcc_from_parameter_sets(&vps, &sps, &pps), None);
}
