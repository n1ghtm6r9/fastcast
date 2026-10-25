use crate::gf::{gf_mul, mul_add_slice};

#[test]
fn mul_add_slice_agrees_with_gf_mul_for_every_byte() {
    let source: Vec<u8> = (0..=255).collect();

    for coefficient in 0..=255u8 {
        let mut target = vec![0u8; 256];

        mul_add_slice(coefficient, &source, &mut target);

        for (index, product) in target.iter().enumerate() {
            assert_eq!(*product, gf_mul(coefficient, index as u8));
        }
    }
}
