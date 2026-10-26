use super::{config_obus_of, frame_obu, real_av1c_records, temporal_delimiter, temporal_unit_of};
use crate::{VideoCodec, split_access_unit};

#[test]
fn every_real_av1c_record_is_rebuilt_byte_for_byte_from_its_sequence_header() {
    for record in real_av1c_records() {
        let temporal_unit = temporal_unit_of(&[
            &temporal_delimiter(),
            config_obus_of(&record.av1c),
            &frame_obu(&[0x10, 0x20, 0x30]),
        ]);

        assert_eq!(
            split_access_unit(VideoCodec::Av1, &temporal_unit).record,
            Some(record.av1c.clone()),
            "{}",
            record.name
        );
    }
}
