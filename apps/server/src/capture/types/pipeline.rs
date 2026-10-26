use crate::capture::VideoEncoder;
use media::VideoCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    pub ffmpeg_path: String,
    pub encoder: VideoEncoder,
    pub codec: VideoCodec,
}
