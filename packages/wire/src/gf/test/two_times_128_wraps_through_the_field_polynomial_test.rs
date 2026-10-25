use crate::gf::gf_mul;

#[test]
fn two_times_128_wraps_through_the_field_polynomial() {
    assert_eq!(gf_mul(2, 128), 0x1D);
}
