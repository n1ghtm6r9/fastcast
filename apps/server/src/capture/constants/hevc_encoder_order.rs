use crate::settings::VideoEncoder;

pub const HEVC_ENCODER_ORDER: [VideoEncoder; 3] =
    [VideoEncoder::Nvenc, VideoEncoder::Amf, VideoEncoder::Qsv];
