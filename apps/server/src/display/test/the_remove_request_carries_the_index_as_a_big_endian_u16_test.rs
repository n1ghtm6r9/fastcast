use crate::display::{IOCTL_INPUT_SIZE, remove_input_of};

#[test]
fn the_remove_request_carries_the_index_as_a_big_endian_u16() {
    let input = remove_input_of(0x0102);

    assert_eq!(input.len(), IOCTL_INPUT_SIZE);
    assert_eq!(input[..2], [0x01, 0x02]);
    assert!(input[2..].iter().all(|byte| *byte == 0));
    assert_eq!(remove_input_of(0), [0; IOCTL_INPUT_SIZE]);
    assert_eq!(remove_input_of(5)[..2], [0x00, 0x05]);
}
