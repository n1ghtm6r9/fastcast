use super::{temporal_delimiter, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn the_last_obu_may_leave_out_its_size() {
    let frame_without_size = [0x30, 0x01, 0x02, 0x03, 0x04];
    let temporal_unit = temporal_unit_of(&[&temporal_delimiter(), &frame_without_size]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        frame_without_size
    );
}
