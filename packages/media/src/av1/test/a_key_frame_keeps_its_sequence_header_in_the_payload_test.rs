use super::{
    config_obus_of, frame_obu, real_av1c_record_named, temporal_delimiter, temporal_unit_of,
};
use crate::{VideoCodec, split_access_unit};

#[test]
fn a_key_frame_keeps_its_sequence_header_in_the_payload() {
    let record = real_av1c_record_named("nvenc_main10_3840x2160_120").av1c;
    let sequence_header = config_obus_of(&record);
    let frame = frame_obu(&[0x10, 0x20, 0x30]);
    let temporal_unit = temporal_unit_of(&[&temporal_delimiter(), sequence_header, &frame]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).payload,
        temporal_unit_of(&[sequence_header, &frame])
    );
}
