use super::{frame_obu, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn a_truncated_obu_ends_the_temporal_unit() {
    let frame = frame_obu(&[0x10, 0x20]);
    let temporal_unit = temporal_unit_of(&[&frame, &[0x32, 10, 0x01, 0x02, 0x03]]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        frame
    );
}
