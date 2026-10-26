use super::{frame_obu, temporal_delimiter, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn the_temporal_delimiter_is_left_out_of_the_payload() {
    let frame = frame_obu(&[0x10, 0x20, 0x30]);
    let temporal_unit = temporal_unit_of(&[&temporal_delimiter(), &frame]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        frame
    );
}
