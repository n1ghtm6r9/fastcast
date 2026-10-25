use crate::capture::codec_name_of;
use crate::settings::VideoEncoder;
use media::VideoCodec;

#[test]
fn every_encoder_has_its_own_ffmpeg_codec_name() {
    let names = [
        (VideoEncoder::Nvenc, VideoCodec::Avc, "h264_nvenc"),
        (VideoEncoder::Amf, VideoCodec::Avc, "h264_amf"),
        (VideoEncoder::Qsv, VideoCodec::Avc, "h264_qsv"),
        (VideoEncoder::X264, VideoCodec::Avc, "libx264"),
        (VideoEncoder::Nvenc, VideoCodec::Hevc, "hevc_nvenc"),
        (VideoEncoder::Amf, VideoCodec::Hevc, "hevc_amf"),
        (VideoEncoder::Qsv, VideoCodec::Hevc, "hevc_qsv"),
        (VideoEncoder::X264, VideoCodec::Hevc, "libx265"),
    ];

    for (encoder, codec, name) in names {
        assert_eq!(
            codec_name_of(encoder, codec),
            name,
            "for {encoder} {codec:?}"
        );
    }
}
