use crate::capture::codecs_to_try;
use crate::settings::CodecChoice;
use media::VideoCodec;

#[test]
fn a_chosen_codec_the_tv_cannot_decode_is_not_tried() {
    assert_eq!(
        codecs_to_try(CodecChoice::Hevc, &[VideoCodec::Avc]),
        vec![VideoCodec::Avc]
    );
}
