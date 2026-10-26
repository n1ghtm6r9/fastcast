use crate::settings::CodecChoice;
use media::VideoCodec;

pub fn native_codec_for(choice: CodecChoice, offered: &[VideoCodec]) -> VideoCodec {
    match choice.codec() {
        Some(chosen) if offered.contains(&chosen) => chosen,
        _ if offered.contains(&VideoCodec::Hevc) => VideoCodec::Hevc,
        _ => VideoCodec::Avc,
    }
}
