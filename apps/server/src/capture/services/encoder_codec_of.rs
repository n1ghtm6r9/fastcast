use media::VideoCodec;
use nvenc::EncoderCodec;

pub fn encoder_codec_of(codec: VideoCodec) -> EncoderCodec {
    match codec {
        VideoCodec::Avc => EncoderCodec::Avc,
        VideoCodec::Hevc => EncoderCodec::Hevc,
    }
}
