use crate::settings::CodecChoice;
use media::VideoCodec;

#[test]
fn a_codec_choice_names_the_codec_it_fixes() {
    assert_eq!(CodecChoice::Auto.codec(), None);
    assert_eq!(CodecChoice::Avc.codec(), Some(VideoCodec::Avc));
    assert_eq!(CodecChoice::Hevc.codec(), Some(VideoCodec::Hevc));
    assert_eq!(CodecChoice::Av1.codec(), Some(VideoCodec::Av1));
}
