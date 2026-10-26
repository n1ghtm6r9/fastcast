use super::{avcc_parameter_sets, real_avcc_records};
use crate::avc::avcc_from_parameter_sets;

#[test]
fn every_real_avcc_record_is_rebuilt_byte_for_byte_from_its_parameter_sets() {
    for record in real_avcc_records() {
        let (sps, pps) = avcc_parameter_sets(&record.avcc);

        assert_eq!(
            avcc_from_parameter_sets(&sps, &pps),
            Some(record.avcc.clone()),
            "{}",
            record.name
        );
    }
}
