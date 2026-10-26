use crate::capture::VideoEncoder;

pub const AUTO_ENCODER_ORDER: [VideoEncoder; 4] = [
    VideoEncoder::Nvenc,
    VideoEncoder::Amf,
    VideoEncoder::Qsv,
    VideoEncoder::X264,
];
