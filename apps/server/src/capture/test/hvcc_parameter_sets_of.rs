pub fn hvcc_parameter_sets_of(hvcc: &[u8]) -> [Vec<u8>; 3] {
    let array_count_at = 22;
    let array_count = hvcc[array_count_at];
    let mut position = array_count_at + 1;
    let mut units = Vec::new();

    for _ in 0..array_count {
        let unit_count = u16::from_be_bytes([hvcc[position + 1], hvcc[position + 2]]);

        position += 3;

        for _ in 0..unit_count {
            let length = usize::from(u16::from_be_bytes([hvcc[position], hvcc[position + 1]]));

            position += 2;
            units.push(hvcc[position..position + length].to_vec());
            position += length;
        }
    }

    units
        .try_into()
        .expect("the record holds a VPS, an SPS and a PPS")
}
