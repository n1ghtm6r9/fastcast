use media::VideoCodec;

pub fn codec_named(name: &str) -> Option<VideoCodec> {
    match name.trim().to_ascii_lowercase().as_str() {
        "hevc" => Some(VideoCodec::Hevc),
        "avc" => Some(VideoCodec::Avc),
        _ => None,
    }
}
