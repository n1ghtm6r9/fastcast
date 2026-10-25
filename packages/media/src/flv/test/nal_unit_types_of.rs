use crate::hevc::hevc_nal_unit_type;

pub fn nal_unit_types_of(length_prefixed_units: &[u8]) -> Vec<u8> {
    let mut types = Vec::new();
    let mut position = 0;

    while position < length_prefixed_units.len() {
        let length_bytes: [u8; 4] = length_prefixed_units[position..position + 4]
            .try_into()
            .expect("a unit starts with a four byte length");

        types.push(hevc_nal_unit_type(length_prefixed_units[position + 4]));
        position += 4 + u32::from_be_bytes(length_bytes) as usize;
    }

    assert_eq!(position, length_prefixed_units.len());

    types
}
