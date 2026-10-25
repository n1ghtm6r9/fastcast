use crate::settings::{EncoderChoice, VideoEncoder};

pub fn encoder_choice_named(name: &str) -> Option<EncoderChoice> {
    match name.trim().to_ascii_lowercase().as_str() {
        "auto" => Some(EncoderChoice::Auto),
        "nvenc" => Some(EncoderChoice::Fixed(VideoEncoder::Nvenc)),
        "amf" => Some(EncoderChoice::Fixed(VideoEncoder::Amf)),
        "qsv" => Some(EncoderChoice::Fixed(VideoEncoder::Qsv)),
        "x264" => Some(EncoderChoice::Fixed(VideoEncoder::X264)),
        _ => None,
    }
}
