use crate::hub::encode_wire_frame;

#[test]
fn a_wire_frame_is_a_flag_byte_a_little_endian_timestamp_and_the_access_unit() {
    let wire = encode_wire_frame(true, 0x0102_0304, &[9, 8, 7]);

    assert_eq!(wire.to_vec(), vec![1, 4, 3, 2, 1, 9, 8, 7]);
}
