use crate::capture::codecs_to_try;
use crate::settings::CodecChoice;
use media::VideoCodec;

#[test]
fn a_chosen_codec_the_tv_decodes_is_tried_first() {
    assert_eq!(
        codecs_to_try(CodecChoice::Avc, &[VideoCodec::Hevc, VideoCodec::Avc]),
        vec![VideoCodec::Avc, VideoCodec::Hevc]
    );
}
