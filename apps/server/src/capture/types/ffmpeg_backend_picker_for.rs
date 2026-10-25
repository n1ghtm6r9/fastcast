use crate::capture::{EncoderPicker, FfmpegBackend};
use media::VideoCodec;

impl FfmpegBackend {
    pub fn picker_for(&self, codec: VideoCodec) -> &EncoderPicker {
        match codec {
            VideoCodec::Avc => &self.avc,
            VideoCodec::Hevc => &self.hevc,
        }
    }
}
