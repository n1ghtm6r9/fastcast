use crate::{BitDepth, YCbCr, ycbcr_of};

#[test]
fn white_is_235_128_128_in_8_bit() {
    assert_eq!(
        ycbcr_of(1.0, 1.0, 1.0, BitDepth::Eight),
        YCbCr {
            y: 235,
            cb: 128,
            cr: 128
        }
    );
}
