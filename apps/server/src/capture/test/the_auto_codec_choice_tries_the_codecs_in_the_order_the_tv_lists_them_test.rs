use crate::capture::codecs_to_try;
use crate::settings::CodecChoice;
use media::VideoCodec;

#[test]
fn the_auto_codec_choice_tries_the_codecs_in_the_order_the_tv_lists_them() {
    assert_eq!(
        codecs_to_try(CodecChoice::Auto, &[VideoCodec::Hevc, VideoCodec::Avc]),
        vec![VideoCodec::Hevc, VideoCodec::Avc]
    );
}
