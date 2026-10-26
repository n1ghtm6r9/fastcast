use crate::gf::{MUL_ADD_KERNEL, mul_add_slice, select_mul_add_kernel};

#[test]
fn mul_add_slice_remembers_the_selected_kernel() {
    mul_add_slice(2, &[1], &mut [0]);

    assert_eq!(MUL_ADD_KERNEL.get(), Some(&select_mul_add_kernel()));
}
