use crate::capture::VideoEncoder;
use media::VideoCodec;

pub fn codec_name_of(encoder: VideoEncoder, codec: VideoCodec) -> &'static str {
    match (encoder, codec) {
        (_, VideoCodec::Av1) => unreachable!("ffmpeg never encodes AV1"),
        (VideoEncoder::Nvenc, VideoCodec::Avc) => "h264_nvenc",
        (VideoEncoder::Nvenc, VideoCodec::Hevc) => "hevc_nvenc",
        (VideoEncoder::Amf, VideoCodec::Avc) => "h264_amf",
        (VideoEncoder::Amf, VideoCodec::Hevc) => "hevc_amf",
        (VideoEncoder::Qsv, VideoCodec::Avc) => "h264_qsv",
        (VideoEncoder::Qsv, VideoCodec::Hevc) => "hevc_qsv",
        (VideoEncoder::X264, VideoCodec::Avc) => "libx264",
        (VideoEncoder::X264, VideoCodec::Hevc) => "libx265",
    }
}
