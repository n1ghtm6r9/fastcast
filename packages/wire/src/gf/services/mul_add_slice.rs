use crate::gf::mul_add_slice_scalar;

pub fn mul_add_slice(coefficient: u8, source: &[u8], target: &mut [u8]) {
    mul_add_slice_scalar(coefficient, source, target);
}
