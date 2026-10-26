use crate::capture::{AMF_ARGS, NVENC_ARGS, QSV_ARGS, QSV_HEVC_ARGS, VideoEncoder, X264_ARGS};
use media::VideoCodec;

pub fn tuning_args_of(encoder: VideoEncoder, codec: VideoCodec) -> &'static str {
    match (encoder, codec) {
        (_, VideoCodec::Av1) => unreachable!("ffmpeg never encodes AV1"),
        (VideoEncoder::Nvenc, _) => NVENC_ARGS,
        (VideoEncoder::Amf, _) => AMF_ARGS,
        (VideoEncoder::Qsv, VideoCodec::Avc) => QSV_ARGS,
        (VideoEncoder::Qsv, VideoCodec::Hevc) => QSV_HEVC_ARGS,
        (VideoEncoder::X264, _) => X264_ARGS,
    }
}
