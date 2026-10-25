use super::flv_video_tag;

pub fn sequence_header_tag(avcc: &[u8]) -> Vec<u8> {
    flv_video_tag(0, true, 0, avcc)
}
