use crate::avc::{VideoSize, first_sps_of_avcc, video_size_from_sps};

pub fn video_size_from_avcc(avcc: &[u8]) -> Option<VideoSize> {
    video_size_from_sps(first_sps_of_avcc(avcc)?)
}
