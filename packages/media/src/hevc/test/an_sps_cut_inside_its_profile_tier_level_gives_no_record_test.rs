use super::{hvcc_parameter_sets, real_hvcc_record_named};
use crate::hevc::hvcc_from_parameter_sets;

#[test]
fn an_sps_cut_inside_its_profile_tier_level_gives_no_record() {
    let (vps, sps, pps) = hvcc_parameter_sets(&real_hvcc_record_named("nvenc_main_1920x1080").hvcc);

    assert_eq!(hvcc_from_parameter_sets(&vps, &sps[..8], &pps), None);
}
