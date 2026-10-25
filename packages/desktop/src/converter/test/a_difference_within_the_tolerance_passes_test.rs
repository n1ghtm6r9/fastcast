use super::assert_close;
use crate::colour::Planes;

#[test]
fn a_difference_within_the_tolerance_passes() {
    let expected = Planes {
        width: 2,
        height: 2,
        luma: vec![10, 20, 30, 40],
        chroma: vec![128, 128],
    };
    let mut actual = expected.clone();
    actual.luma[1] = 21;
    actual.chroma[0] = 127;

    assert_close(&actual, &expected, 1);
}
