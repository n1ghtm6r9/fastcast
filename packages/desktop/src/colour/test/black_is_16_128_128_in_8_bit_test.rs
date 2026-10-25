use crate::{BitDepth, YCbCr, ycbcr_of};

#[test]
fn black_is_16_128_128_in_8_bit() {
    assert_eq!(
        ycbcr_of(0.0, 0.0, 0.0, BitDepth::Eight),
        YCbCr {
            y: 16,
            cb: 128,
            cr: 128
        }
    );
}
