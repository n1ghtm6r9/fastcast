use crate::avc::VideoSize;

pub fn video_size(width: u32, height: u32) -> VideoSize {
    VideoSize { width, height }
}
