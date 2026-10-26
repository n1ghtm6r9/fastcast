use super::annex_b_of;
use crate::VideoCodec;
use crate::annexb::{length_prefixed_units, split_access_unit};

#[test]
fn a_frame_without_parameter_sets_has_no_record() {
    let slice = [0x41, 0x9A];
    let annex_b = annex_b_of(&[&slice]);

    let access_unit = split_access_unit(VideoCodec::Avc, &annex_b);

    assert_eq!(access_unit.record, None);
    assert_eq!(access_unit.payload, length_prefixed_units(&[&slice]));
}
