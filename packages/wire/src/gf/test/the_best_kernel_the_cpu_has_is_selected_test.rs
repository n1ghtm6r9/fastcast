use crate::gf::{MulAddKernel, select_mul_add_kernel};

#[test]
fn the_best_kernel_the_cpu_has_is_selected() {
    let expected = if is_x86_feature_detected!("avx2") {
        MulAddKernel::Avx2
    } else if is_x86_feature_detected!("ssse3") {
        MulAddKernel::Ssse3
    } else {
        MulAddKernel::Scalar
    };

    assert_eq!(select_mul_add_kernel(), expected);
}
