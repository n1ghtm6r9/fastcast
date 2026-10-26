use crate::annexb::AccessUnit;
use crate::av1::{
    OBU_PADDING, OBU_SEQUENCE_HEADER, OBU_TEMPORAL_DELIMITER, av1c_from_sequence_header, obus_of,
};

pub fn split_temporal_unit(temporal_unit: &[u8]) -> AccessUnit {
    let mut record = None;
    let mut payload = Vec::with_capacity(temporal_unit.len());

    for obu in obus_of(temporal_unit) {
        match obu.kind {
            OBU_TEMPORAL_DELIMITER | OBU_PADDING => continue,
            OBU_SEQUENCE_HEADER if record.is_none() => record = av1c_from_sequence_header(&obu),
            _ => {}
        }

        payload.extend_from_slice(obu.bytes);
    }

    AccessUnit { record, payload }
}
