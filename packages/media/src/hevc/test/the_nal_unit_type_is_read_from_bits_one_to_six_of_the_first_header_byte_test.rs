use crate::hevc::hevc_nal_unit_type;

#[test]
fn the_nal_unit_type_is_read_from_bits_one_to_six_of_the_first_header_byte() {
    assert_eq!(hevc_nal_unit_type(0x00), 0);
    assert_eq!(hevc_nal_unit_type(0x02), 1);
    assert_eq!(hevc_nal_unit_type(0x26), 19);
    assert_eq!(hevc_nal_unit_type(0x40), 32);
    assert_eq!(hevc_nal_unit_type(0x42), 33);
    assert_eq!(hevc_nal_unit_type(0x44), 34);
    assert_eq!(hevc_nal_unit_type(0x4e), 39);
}
