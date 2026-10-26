use super::annex_b_of;
use crate::VideoCodec;
use crate::annexb::{length_prefixed_units, split_access_unit};
use crate::avc::test::{avcc_parameter_sets, real_avcc_record_named};

#[test]
fn an_sps_without_a_pps_gives_no_record() {
    let (sps, _) = avcc_parameter_sets(&real_avcc_record_named("nvenc_high_1920x1080").avcc);
    let idr = [0x65, 0x88, 0x84];
    let annex_b = annex_b_of(&[&sps, &idr]);

    let access_unit = split_access_unit(VideoCodec::Avc, &annex_b);

    assert_eq!(access_unit.record, None);
    assert_eq!(access_unit.payload, length_prefixed_units(&[&idr]));
}
