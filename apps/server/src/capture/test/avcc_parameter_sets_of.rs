pub fn avcc_parameter_sets_of(avcc: &[u8]) -> [Vec<u8>; 2] {
    let sps_length_at = 6;
    let sps_length = usize::from(u16::from_be_bytes([
        avcc[sps_length_at],
        avcc[sps_length_at + 1],
    ]));
    let sps_at = sps_length_at + 2;
    let pps_length_at = sps_at + sps_length + 1;
    let pps_length = usize::from(u16::from_be_bytes([
        avcc[pps_length_at],
        avcc[pps_length_at + 1],
    ]));
    let pps_at = pps_length_at + 2;

    [
        avcc[sps_at..sps_at + sps_length].to_vec(),
        avcc[pps_at..pps_at + pps_length].to_vec(),
    ]
}
