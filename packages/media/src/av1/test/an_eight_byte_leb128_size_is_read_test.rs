use super::{frame_obu, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn an_eight_byte_leb128_size_is_read() {
    let padded_size = [
        0x32, 0x83, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x01, 0x02, 0x03,
    ];
    let temporal_unit = temporal_unit_of(&[&padded_size, &frame_obu(&[0x04])]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        temporal_unit
    );
}
