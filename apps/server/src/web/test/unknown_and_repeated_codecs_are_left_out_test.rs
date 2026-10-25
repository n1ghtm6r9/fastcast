use crate::web::codecs_from;
use media::VideoCodec;

#[test]
fn unknown_and_repeated_codecs_are_left_out() {
    assert_eq!(
        codecs_from(Some("av1, HEVC,hevc,,avc,Avc")),
        vec![VideoCodec::Hevc, VideoCodec::Avc]
    );
}
