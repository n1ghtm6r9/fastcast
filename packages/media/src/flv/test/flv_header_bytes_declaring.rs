pub fn flv_header_bytes_declaring(declared_length: u32) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(b"FLV");
    bytes.push(1);
    bytes.push(0x01);
    bytes.extend_from_slice(&declared_length.to_be_bytes());
    bytes.resize(declared_length as usize, 0);
    bytes.extend_from_slice(&[0, 0, 0, 0]);

    bytes
}
