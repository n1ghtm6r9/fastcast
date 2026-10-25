use crate::gf::mul_add_slice;

#[test]
fn mul_add_slice_xors_the_scaled_source_into_the_target() {
    let source = [1, 2, 3];
    let mut target = [1, 2, 3];

    mul_add_slice(2, &source, &mut target);

    assert_eq!(target, [3, 6, 5]);
}
