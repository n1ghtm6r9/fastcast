use super::{frame_obu, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn a_leb128_size_longer_than_eight_bytes_ends_the_temporal_unit() {
    let frame = frame_obu(&[0x01]);
    let oversized = [
        0x32, 0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x01,
    ];
    let temporal_unit = temporal_unit_of(&[&frame, &oversized]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        frame
    );
}
