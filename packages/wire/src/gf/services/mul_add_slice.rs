use crate::gf::mul_add_slice_scalar;
#[cfg(target_arch = "x86_64")]
use crate::gf::{MUL_ADD_KERNEL, MulAddKernel, mul_add_slice_ssse3, select_mul_add_kernel};

pub fn mul_add_slice(coefficient: u8, source: &[u8], target: &mut [u8]) {
    #[cfg(target_arch = "x86_64")]
    {
        match MUL_ADD_KERNEL.get_or_init(select_mul_add_kernel) {
            MulAddKernel::Ssse3 => {
                return unsafe { mul_add_slice_ssse3(coefficient, source, target) };
            }
            MulAddKernel::Scalar => {}
        }
    }

    mul_add_slice_scalar(coefficient, source, target);
}
