pub fn avcc_with_sps_units(sps_units: &[Vec<u8>]) -> Vec<u8> {
    let mut avcc = vec![1, 100, 0, 42, 0xff, 0xe0 | sps_units.len() as u8];

    for sps in sps_units {
        avcc.extend_from_slice(&(sps.len() as u16).to_be_bytes());
        avcc.extend_from_slice(sps);
    }

    avcc.push(0);

    avcc
}
