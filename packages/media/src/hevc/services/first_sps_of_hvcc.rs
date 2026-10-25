use crate::hevc::{
    HEVC_SPS_NAL_UNIT_TYPE, HVCC_ARRAY_COUNT_OFFSET, HVCC_ARRAY_HEADER_LENGTH,
    HVCC_ARRAY_NAL_UNIT_TYPE_MASK, HVCC_FIRST_ARRAY_OFFSET, HVCC_NAL_UNIT_LENGTH_FIELD_SIZE,
    length_prefixed_unit_at, u16_at,
};

pub fn first_sps_of_hvcc(hvcc: &[u8]) -> Option<&[u8]> {
    let array_count = *hvcc.get(HVCC_ARRAY_COUNT_OFFSET)?;
    let mut position = HVCC_FIRST_ARRAY_OFFSET;

    for _ in 0..array_count {
        let nal_unit_type = hvcc.get(position)? & HVCC_ARRAY_NAL_UNIT_TYPE_MASK;
        let unit_count = u16_at(hvcc, position + 1)?;

        position += HVCC_ARRAY_HEADER_LENGTH;

        for _ in 0..unit_count {
            let unit = length_prefixed_unit_at(hvcc, position)?;

            if nal_unit_type == HEVC_SPS_NAL_UNIT_TYPE {
                return Some(unit);
            }

            position += HVCC_NAL_UNIT_LENGTH_FIELD_SIZE + unit.len();
        }
    }

    None
}
