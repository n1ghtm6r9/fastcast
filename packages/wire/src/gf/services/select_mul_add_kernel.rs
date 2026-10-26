use crate::gf::MulAddKernel;

pub fn select_mul_add_kernel() -> MulAddKernel {
    if is_x86_feature_detected!("ssse3") {
        MulAddKernel::Ssse3
    } else {
        MulAddKernel::Scalar
    }
}
