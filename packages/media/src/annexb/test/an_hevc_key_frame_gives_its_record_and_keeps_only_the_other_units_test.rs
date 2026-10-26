use super::annex_b_of;
use crate::VideoCodec;
use crate::annexb::{length_prefixed_units, split_access_unit};
use crate::hevc::test::{hvcc_parameter_sets, real_hvcc_record_named};

#[test]
fn an_hevc_key_frame_gives_its_record_and_keeps_only_the_other_units() {
    let record = real_hvcc_record_named("nvenc_main_3840x2160").hvcc;
    let (vps, sps, pps) = hvcc_parameter_sets(&record);
    let access_unit_delimiter = [0x46, 0x01, 0x50];
    let idr = [0x26, 0x01, 0xAF];
    let annex_b = annex_b_of(&[&access_unit_delimiter, &vps, &sps, &pps, &idr]);

    let access_unit = split_access_unit(VideoCodec::Hevc, &annex_b);

    assert_eq!(access_unit.record, Some(record));
    assert_eq!(access_unit.payload, length_prefixed_units(&[&idr]));
}
