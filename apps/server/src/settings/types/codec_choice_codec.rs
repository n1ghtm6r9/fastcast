use crate::settings::CodecChoice;
use media::VideoCodec;

impl CodecChoice {
    pub fn codec(self) -> Option<VideoCodec> {
        match self {
            CodecChoice::Auto => None,
            CodecChoice::Avc => Some(VideoCodec::Avc),
            CodecChoice::Hevc => Some(VideoCodec::Hevc),
        }
    }
}
