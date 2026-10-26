use super::{frame_obu, obu_of, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn padding_is_left_out_of_the_payload() {
    let frame = frame_obu(&[0x10, 0x20]);
    let temporal_unit = temporal_unit_of(&[&frame, &obu_of(15, &[0, 0, 0])]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        frame
    );
}
