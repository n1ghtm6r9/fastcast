use crate::hevc::hevc_nal_unit_type;

#[test]
fn the_layer_id_and_the_forbidden_bit_do_not_change_the_nal_unit_type() {
    assert_eq!(hevc_nal_unit_type(0x43), 33);
    assert_eq!(hevc_nal_unit_type(0xc2), 33);
    assert_eq!(hevc_nal_unit_type(0xc3), 33);
}
