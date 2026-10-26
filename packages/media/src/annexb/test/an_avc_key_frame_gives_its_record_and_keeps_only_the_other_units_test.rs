use super::annex_b_of;
use crate::VideoCodec;
use crate::annexb::{length_prefixed_units, split_access_unit};
use crate::avc::test::{avcc_parameter_sets, real_avcc_record_named};

#[test]
fn an_avc_key_frame_gives_its_record_and_keeps_only_the_other_units() {
    let record = real_avcc_record_named("nvenc_high_1920x1080").avcc;
    let (sps, pps) = avcc_parameter_sets(&record);
    let access_unit_delimiter = [0x09, 0xF0];
    let sei = [0x06, 0x05, 0x01, 0x80];
    let idr = [0x65, 0x88, 0x84];
    let annex_b = annex_b_of(&[&access_unit_delimiter, &sps, &pps, &sei, &idr]);

    let access_unit = split_access_unit(VideoCodec::Avc, &annex_b);

    assert_eq!(access_unit.record, Some(record));
    assert_eq!(access_unit.payload, length_prefixed_units(&[&sei, &idr]));
}
