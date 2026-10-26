use crate::capture::encoder_codec_of;
use media::VideoCodec;
use nvenc::EncoderCodec;

#[test]
fn the_encoder_codec_follows_the_stream_codec() {
    assert_eq!(encoder_codec_of(VideoCodec::Avc), EncoderCodec::Avc);
    assert_eq!(encoder_codec_of(VideoCodec::Hevc), EncoderCodec::Hevc);
}
