use crate::avc::{AVCC_FIRST_SPS_LENGTH_OFFSET, AVCC_FIRST_SPS_OFFSET};

pub fn avcc_parameter_sets(avcc: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let sps_length = usize::from(u16::from_be_bytes([
        avcc[AVCC_FIRST_SPS_LENGTH_OFFSET],
        avcc[AVCC_FIRST_SPS_LENGTH_OFFSET + 1],
    ]));
    let sps_end = AVCC_FIRST_SPS_OFFSET + sps_length;
    let sps = avcc[AVCC_FIRST_SPS_OFFSET..sps_end].to_vec();

    let pps_length_offset = sps_end + 1;
    let pps_length = usize::from(u16::from_be_bytes([
        avcc[pps_length_offset],
        avcc[pps_length_offset + 1],
    ]));
    let pps_start = pps_length_offset + 2;
    let pps = avcc[pps_start..pps_start + pps_length].to_vec();

    (sps, pps)
}
