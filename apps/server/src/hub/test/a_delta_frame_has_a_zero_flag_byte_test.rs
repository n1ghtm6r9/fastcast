use crate::hub::encode_wire_frame;

#[test]
fn a_delta_frame_has_a_zero_flag_byte() {
    let wire = encode_wire_frame(false, 0, &[5]);

    assert_eq!(wire.to_vec(), vec![0, 0, 0, 0, 0, 5]);
}
