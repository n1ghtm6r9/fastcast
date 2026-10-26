use super::{
    config_obus_of, frame_obu, real_av1c_record_named, temporal_delimiter, temporal_unit_of,
};
use crate::{VideoCodec, split_access_unit};

#[test]
fn a_temporal_unit_without_a_sequence_header_has_no_record() {
    let record = real_av1c_record_named("nvenc_main_1920x1080").av1c;
    let bytes_of_a_sequence_header = &config_obus_of(&record)[2..];
    let temporal_unit = temporal_unit_of(&[
        &temporal_delimiter(),
        &frame_obu(bytes_of_a_sequence_header),
    ]);

    assert_eq!(
        split_access_unit(VideoCodec::Av1, &temporal_unit).record,
        None
    );
}
