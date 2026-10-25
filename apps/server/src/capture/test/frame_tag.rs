use super::flv_video_tag;

pub fn frame_tag(timestamp_ms: u32, key: bool, nal_payload: &[u8]) -> Vec<u8> {
    let mut payload = (nal_payload.len() as u32).to_be_bytes().to_vec();

    payload.extend_from_slice(nal_payload);

    flv_video_tag(timestamp_ms, key, 1, &payload)
}
