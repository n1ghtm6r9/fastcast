use crate::avc::VideoSize;
use crate::hevc::{first_sps_of_hvcc, video_size_from_hevc_sps};

pub fn video_size_from_hvcc(hvcc: &[u8]) -> Option<VideoSize> {
    video_size_from_hevc_sps(first_sps_of_hvcc(hvcc)?)
}
