use crate::capture::converter_depth_of;
use crate::hub::{EIGHT_BIT_DEPTH, TEN_BIT_DEPTH};
use desktop::BitDepth;

#[test]
fn ten_bits_ask_the_converter_for_ten_bit_output() {
    assert_eq!(converter_depth_of(TEN_BIT_DEPTH), BitDepth::Ten);
    assert_eq!(converter_depth_of(EIGHT_BIT_DEPTH), BitDepth::Eight);
}
