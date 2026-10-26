use crate::gf::{gf_mul, mul_add_slice_scalar};
use crate::test_support::random_bytes;

#[test]
fn the_scalar_kernel_agrees_with_gf_mul_for_every_coefficient_and_byte() {
    let source: Vec<u8> = (0..=255).collect();
    let mut state = 0x9E37_79B9_7F4A_7C15;
    let initial = random_bytes(&mut state, 256);

    for coefficient in 0..=255u8 {
        let mut target = initial.clone();

        mul_add_slice_scalar(coefficient, &source, &mut target);

        for (index, byte) in target.iter().enumerate() {
            assert_eq!(*byte, initial[index] ^ gf_mul(coefficient, source[index]));
        }
    }
}
