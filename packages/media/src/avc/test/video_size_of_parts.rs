use super::SpsParts;
use crate::avc::{VideoSize, video_size_from_sps};

pub fn video_size_of_parts(parts: &SpsParts) -> Option<VideoSize> {
    video_size_from_sps(&parts.nal_unit())
}
