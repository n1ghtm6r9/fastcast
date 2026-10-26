use crate::capture::{EncoderPicker, FfmpegBackend};
use media::VideoCodec;

impl FfmpegBackend {
    pub fn picker_for(&self, codec: VideoCodec) -> Option<&EncoderPicker> {
        match codec {
            VideoCodec::Avc => Some(&self.avc),
            VideoCodec::Hevc => Some(&self.hevc),
            VideoCodec::Av1 => None,
        }
    }
}
