use crate::gf::{gf_inv, gf_mul};

#[test]
fn every_non_zero_element_times_its_inverse_is_one() {
    assert!((1..=255).all(|value| gf_mul(value, gf_inv(value)) == 1));
}
