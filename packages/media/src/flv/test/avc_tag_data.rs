pub fn avc_tag_data(frame_type: u8, packet_type: u8, payload: &[u8]) -> Vec<u8> {
    let mut data = vec![(frame_type << 4) | 7, packet_type, 0, 0, 0];

    data.extend_from_slice(payload);

    data
}
