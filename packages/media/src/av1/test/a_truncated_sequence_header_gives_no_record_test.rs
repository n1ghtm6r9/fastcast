use super::{config_obus_of, obu_of, real_av1c_record_named, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn a_truncated_sequence_header_gives_no_record() {
    let av1c = real_av1c_record_named("nvenc_main10_3840x2160_120").av1c;
    let payload = &config_obus_of(&av1c)[2..];
    let truncated = obu_of(1, &payload[..10]);

    let access_unit = split_access_unit(VideoCodec::Av1, &temporal_unit_of(&[&truncated]));

    assert_eq!(access_unit.record, None);
    assert_eq!(access_unit.payload, truncated);
}
