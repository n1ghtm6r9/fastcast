use super::{frame_obu, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn a_size_written_in_several_leb128_bytes_is_read_whole() {
    let long = frame_obu(&[0x55; 300]);
    let short = frame_obu(&[0x66]);
    let temporal_unit = temporal_unit_of(&[&long, &short]);

    assert_eq!(long[1..3], [0xAC, 0x02]);
    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        temporal_unit
    );
}
