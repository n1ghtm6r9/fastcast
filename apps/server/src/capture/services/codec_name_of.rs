use crate::settings::VideoEncoder;

pub fn codec_name_of(encoder: VideoEncoder) -> &'static str {
    match encoder {
        VideoEncoder::Nvenc => "h264_nvenc",
        VideoEncoder::Amf => "h264_amf",
        VideoEncoder::Qsv => "h264_qsv",
        VideoEncoder::X264 => "libx264",
    }
}
