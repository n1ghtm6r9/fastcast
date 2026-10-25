use crate::gf::GF_EXP;

#[test]
fn the_generator_two_has_order_255() {
    assert_eq!(GF_EXP[255], 1);
    assert!((1..255).all(|index| GF_EXP[index] != 1));
}
