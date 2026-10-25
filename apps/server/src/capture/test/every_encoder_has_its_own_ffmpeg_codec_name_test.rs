use crate::capture::codec_name_of;
use crate::settings::VideoEncoder;

#[test]
fn every_encoder_has_its_own_ffmpeg_codec_name() {
    assert_eq!(codec_name_of(VideoEncoder::Nvenc), "h264_nvenc");
    assert_eq!(codec_name_of(VideoEncoder::Amf), "h264_amf");
    assert_eq!(codec_name_of(VideoEncoder::Qsv), "h264_qsv");
    assert_eq!(codec_name_of(VideoEncoder::X264), "libx264");
}
