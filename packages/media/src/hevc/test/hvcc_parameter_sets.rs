use crate::hevc::{
    HVCC_ARRAY_COUNT_OFFSET, HVCC_ARRAY_HEADER_LENGTH, HVCC_FIRST_ARRAY_OFFSET,
    HVCC_NAL_UNIT_LENGTH_FIELD_SIZE, length_prefixed_unit_at,
};

pub fn hvcc_parameter_sets(hvcc: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let array_count = hvcc[HVCC_ARRAY_COUNT_OFFSET];
    let mut position = HVCC_FIRST_ARRAY_OFFSET;
    let mut units = Vec::new();

    for _ in 0..array_count {
        position += HVCC_ARRAY_HEADER_LENGTH;

        let unit = length_prefixed_unit_at(hvcc, position).expect("the array holds its unit");

        position += HVCC_NAL_UNIT_LENGTH_FIELD_SIZE + unit.len();
        units.push(unit.to_vec());
    }

    let [vps, sps, pps]: [Vec<u8>; 3] = units
        .try_into()
        .expect("the record holds a VPS, an SPS and a PPS");

    (vps, sps, pps)
}
