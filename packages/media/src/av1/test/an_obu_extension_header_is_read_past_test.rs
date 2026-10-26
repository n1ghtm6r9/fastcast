use super::temporal_unit_of;
use crate::{VideoCodec, split_access_unit};

#[test]
fn an_obu_extension_header_is_read_past() {
    let delimiter = [0x16, 0x28, 0x00];
    let frame = [0x36, 0x28, 0x02, 0xAA, 0xBB];
    let temporal_unit = temporal_unit_of(&[&delimiter, &frame]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        frame
    );
}
