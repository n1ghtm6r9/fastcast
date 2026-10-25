use crate::{BitDepth, YCbCr, ycbcr_of};

#[test]
fn bt709_primaries_map_to_their_8_bit_codes() {
    assert_eq!(
        ycbcr_of(1.0, 0.0, 0.0, BitDepth::Eight),
        YCbCr {
            y: 63,
            cb: 102,
            cr: 240
        }
    );
    assert_eq!(
        ycbcr_of(0.0, 1.0, 0.0, BitDepth::Eight),
        YCbCr {
            y: 173,
            cb: 42,
            cr: 26
        }
    );
    assert_eq!(
        ycbcr_of(0.0, 0.0, 1.0, BitDepth::Eight),
        YCbCr {
            y: 32,
            cb: 240,
            cr: 118
        }
    );
}
