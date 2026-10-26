use super::{hvcc_parameter_sets, real_hvcc_records};
use crate::hevc::hvcc_from_parameter_sets;

#[test]
fn every_real_hvcc_record_is_rebuilt_byte_for_byte_from_its_parameter_sets() {
    for record in real_hvcc_records() {
        let (vps, sps, pps) = hvcc_parameter_sets(&record.hvcc);

        assert_eq!(
            hvcc_from_parameter_sets(&vps, &sps, &pps),
            Some(record.hvcc.clone()),
            "{}",
            record.name
        );
    }
}
