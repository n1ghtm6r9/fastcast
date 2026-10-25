pub fn flv_video_tag(timestamp_ms: u32, key: bool, packet_type: u8, payload: &[u8]) -> Vec<u8> {
    let frame_type: u8 = if key { 1 } else { 2 };
    let mut data = vec![(frame_type << 4) | 7, packet_type, 0, 0, 0];

    data.extend_from_slice(payload);

    let size = data.len() as u32;
    let timestamp = timestamp_ms.to_be_bytes();
    let mut tag = vec![9];

    tag.extend_from_slice(&size.to_be_bytes()[1..]);
    tag.extend_from_slice(&timestamp[1..]);
    tag.push(timestamp[0]);
    tag.extend_from_slice(&[0, 0, 0]);
    tag.extend_from_slice(&data);
    tag.extend_from_slice(&(11 + size).to_be_bytes());

    tag
}
