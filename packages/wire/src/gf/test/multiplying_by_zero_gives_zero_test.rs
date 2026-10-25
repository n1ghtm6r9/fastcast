use crate::gf::gf_mul;

#[test]
fn multiplying_by_zero_gives_zero() {
    for value in 0..=255 {
        assert_eq!(gf_mul(0, value), 0);
        assert_eq!(gf_mul(value, 0), 0);
    }
}
