use crate::colour::Planes;

pub fn assert_close(actual: &Planes, expected: &Planes, tolerance: u16) {
    assert_eq!(
        (actual.width, actual.height),
        (expected.width, expected.height)
    );

    for (name, actual, expected) in [
        ("luma", &actual.luma, &expected.luma),
        ("chroma", &actual.chroma, &expected.chroma),
    ] {
        assert_eq!(actual.len(), expected.len(), "{name} sample count");

        for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
            assert!(
                actual.abs_diff(*expected) <= tolerance,
                "{name} sample {index}: gpu {actual}, reference {expected}"
            );
        }
    }
}
