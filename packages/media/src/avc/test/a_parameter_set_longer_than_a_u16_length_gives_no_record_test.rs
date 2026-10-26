use super::{avcc_parameter_sets, real_avcc_record_named};
use crate::avc::avcc_from_parameter_sets;

#[test]
fn a_parameter_set_longer_than_a_u16_length_gives_no_record() {
    let (sps, _) = avcc_parameter_sets(&real_avcc_record_named("nvenc_high_1920x1080").avcc);
    let pps = vec![0x68; usize::from(u16::MAX) + 1];

    assert_eq!(avcc_from_parameter_sets(&sps, &pps), None);
}
