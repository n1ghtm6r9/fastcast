use crate::fec::cauchy_coefficient;
use crate::gf::gf_inv;

#[test]
fn the_cauchy_coefficient_is_the_inverse_of_x_xor_y() {
    assert_eq!(cauchy_coefficient(0, 0), gf_inv(128));
    assert_eq!(cauchy_coefficient(2, 5), gf_inv(130 ^ 5));
}
