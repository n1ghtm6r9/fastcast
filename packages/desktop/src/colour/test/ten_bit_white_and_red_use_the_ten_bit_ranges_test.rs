use crate::{BitDepth, YCbCr, ycbcr_of};

#[test]
fn ten_bit_white_and_red_use_the_ten_bit_ranges() {
    assert_eq!(
        ycbcr_of(1.0, 1.0, 1.0, BitDepth::Ten),
        YCbCr {
            y: 940,
            cb: 512,
            cr: 512
        }
    );
    assert_eq!(
        ycbcr_of(1.0, 0.0, 0.0, BitDepth::Ten),
        YCbCr {
            y: 250,
            cb: 409,
            cr: 960
        }
    );
}
