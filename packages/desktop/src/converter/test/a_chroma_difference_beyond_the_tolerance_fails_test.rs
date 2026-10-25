use super::assert_close;
use crate::colour::Planes;

#[test]
#[should_panic(expected = "chroma sample 0")]
fn a_chroma_difference_beyond_the_tolerance_fails() {
    let expected = Planes {
        width: 2,
        height: 2,
        luma: vec![10, 20, 30, 40],
        chroma: vec![128, 128],
    };
    let mut actual = expected.clone();
    actual.chroma[0] = 126;

    assert_close(&actual, &expected, 1);
}
