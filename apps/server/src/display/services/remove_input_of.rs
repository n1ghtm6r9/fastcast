use crate::display::{EMPTY_IOCTL_INPUT, IOCTL_INPUT_SIZE};

pub fn remove_input_of(index: u16) -> [u8; IOCTL_INPUT_SIZE] {
    let mut input = EMPTY_IOCTL_INPUT;

    input[..size_of::<u16>()].copy_from_slice(&index.to_be_bytes());

    input
}
