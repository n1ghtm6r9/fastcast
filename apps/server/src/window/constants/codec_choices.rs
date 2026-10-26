use crate::settings::CodecChoice;

pub const CODEC_CHOICES: [(CodecChoice, &str); 4] = [
    (CodecChoice::Auto, "Авто"),
    (CodecChoice::Avc, "H.264"),
    (CodecChoice::Hevc, "HEVC"),
    (CodecChoice::Av1, "AV1"),
];
