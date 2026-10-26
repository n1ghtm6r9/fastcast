use crate::avc::avcc_from_parameter_sets;

#[test]
fn an_sps_too_short_to_hold_a_level_gives_no_record() {
    assert_eq!(
        avcc_from_parameter_sets(&[0x67, 0x64, 0x00], &[0x68, 0xCE]),
        None
    );
}
