use super::{obu_of, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn frame_headers_tile_groups_and_metadata_stay_in_the_payload_in_order() {
    let temporal_unit = temporal_unit_of(&[
        &obu_of(5, &[0x01, 0x02]),
        &obu_of(3, &[0x03]),
        &obu_of(4, &[0x04, 0x05]),
    ]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        temporal_unit
    );
}
