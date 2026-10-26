#[cfg(target_arch = "x86_64")]
use crate::gf::mul_add_slice_ssse3;
use crate::gf::test::KernelFunction;
use crate::gf::{mul_add_slice, mul_add_slice_scalar};

pub fn available_kernels() -> Vec<(&'static str, KernelFunction)> {
    let candidates: &[(&str, bool, KernelFunction)] = &[
        ("scalar", true, mul_add_slice_scalar),
        #[cfg(target_arch = "x86_64")]
        (
            "ssse3",
            is_x86_feature_detected!("ssse3"),
            mul_add_slice_ssse3,
        ),
        ("mul_add_slice", true, mul_add_slice),
    ];

    candidates
        .iter()
        .filter(|(_, available, _)| *available)
        .map(|&(name, _, kernel)| (name, kernel))
        .collect()
}
