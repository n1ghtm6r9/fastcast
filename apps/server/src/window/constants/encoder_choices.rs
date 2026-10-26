use crate::settings::{EncoderChoice, VideoEncoder};

pub const ENCODER_CHOICES: [(EncoderChoice, &str); 5] = [
    (EncoderChoice::Auto, "Авто"),
    (EncoderChoice::Fixed(VideoEncoder::Nvenc), "NVENC"),
    (EncoderChoice::Fixed(VideoEncoder::Amf), "AMF"),
    (EncoderChoice::Fixed(VideoEncoder::Qsv), "QSV"),
    (EncoderChoice::Fixed(VideoEncoder::X264), "x264"),
];
