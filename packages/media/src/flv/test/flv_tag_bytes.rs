pub fn flv_tag_bytes(kind: u8, timestamp_ms: u32, data: &[u8]) -> Vec<u8> {
    let size = data.len() as u32;
    let timestamp = timestamp_ms.to_be_bytes();
    let mut bytes = vec![kind];

    bytes.extend_from_slice(&size.to_be_bytes()[1..]);
    bytes.extend_from_slice(&timestamp[1..]);
    bytes.push(timestamp[0]);
    bytes.extend_from_slice(&[0, 0, 0]);
    bytes.extend_from_slice(data);
    bytes.extend_from_slice(&(11 + size).to_be_bytes());

    bytes
}
