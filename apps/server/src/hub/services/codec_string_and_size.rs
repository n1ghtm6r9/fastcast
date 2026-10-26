use media::{
    VideoCodec, VideoSize, av1_codec_string, avc_codec_string, hevc_codec_string,
    video_size_from_av1c, video_size_from_avcc, video_size_from_hvcc,
};

pub fn codec_string_and_size(codec: VideoCodec, record: &[u8]) -> Option<(String, VideoSize)> {
    match codec {
        VideoCodec::Avc => Some((avc_codec_string(record)?, video_size_from_avcc(record)?)),
        VideoCodec::Hevc => Some((hevc_codec_string(record)?, video_size_from_hvcc(record)?)),
        VideoCodec::Av1 => Some((av1_codec_string(record)?, video_size_from_av1c(record)?)),
    }
}
