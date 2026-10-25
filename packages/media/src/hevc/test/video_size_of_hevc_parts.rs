use super::HevcSpsParts;
use crate::avc::VideoSize;
use crate::hevc::video_size_from_hevc_sps;

pub fn video_size_of_hevc_parts(parts: &HevcSpsParts) -> Option<VideoSize> {
    video_size_from_hevc_sps(&parts.nal_unit())
}
