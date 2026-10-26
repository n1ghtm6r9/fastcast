use crate::VideoCodec;
use crate::annexb::{
    AccessUnit, NalUnitKind, annex_b_units, length_prefixed_units, nal_unit_kind_of,
    record_from_parameter_sets,
};

pub fn split_access_unit(codec: VideoCodec, annex_b: &[u8]) -> AccessUnit {
    let mut vps = None;
    let mut sps = None;
    let mut pps = None;
    let mut payload_units = Vec::new();

    for unit in annex_b_units(annex_b) {
        match nal_unit_kind_of(codec, unit) {
            NalUnitKind::Vps => {
                vps.get_or_insert(unit);
            }
            NalUnitKind::Sps => {
                sps.get_or_insert(unit);
            }
            NalUnitKind::Pps => {
                pps.get_or_insert(unit);
            }
            NalUnitKind::AccessUnitDelimiter => {}
            NalUnitKind::Other => payload_units.push(unit),
        }
    }

    AccessUnit {
        record: record_from_parameter_sets(codec, vps, sps, pps),
        payload: length_prefixed_units(&payload_units),
    }
}
