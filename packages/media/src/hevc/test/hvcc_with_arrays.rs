use super::hvcc_header;

pub fn hvcc_with_arrays(arrays: &[(u8, Vec<Vec<u8>>)]) -> Vec<u8> {
    let mut hvcc = hvcc_header(0x01, 0x6000_0000, [0x90, 0, 0, 0, 0, 0], 120);

    hvcc.push(arrays.len() as u8);

    for (nal_unit_type, units) in arrays {
        hvcc.push(0x80 | nal_unit_type);
        hvcc.extend_from_slice(&(units.len() as u16).to_be_bytes());

        for unit in units {
            hvcc.extend_from_slice(&(unit.len() as u16).to_be_bytes());
            hvcc.extend_from_slice(unit);
        }
    }

    hvcc
}
