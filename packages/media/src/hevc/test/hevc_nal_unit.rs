pub fn hevc_nal_unit(nal_unit_type: u8, payload: &[u8]) -> Vec<u8> {
    let mut nal_unit = vec![nal_unit_type << 1, 1];

    nal_unit.extend_from_slice(payload);

    nal_unit
}
