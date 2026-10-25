pub fn enhanced_tag_data(
    frame_type: u8,
    packet_type: u8,
    fourcc: &[u8; 4],
    payload: &[u8],
) -> Vec<u8> {
    let mut data = vec![0x80 | (frame_type << 4) | packet_type];

    data.extend_from_slice(fourcc);
    data.extend_from_slice(payload);

    data
}
