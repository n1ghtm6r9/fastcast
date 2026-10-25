use super::assert_close;
use crate::colour::Planes;

#[test]
#[should_panic(expected = "luma sample 1")]
fn a_luma_difference_beyond_the_tolerance_fails() {
    let expected = Planes {
        width: 2,
        height: 2,
        luma: vec![10, 20, 30, 40],
        chroma: vec![128, 128],
    };
    let mut actual = expected.clone();
    actual.luma[1] = 22;

    assert_close(&actual, &expected, 1);
}
