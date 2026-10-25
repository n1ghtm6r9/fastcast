use super::environment_with;
use crate::settings::{EncoderChoice, VideoEncoder, settings_from};

#[test]
fn encoder_names_are_read_in_any_case_and_with_spaces_around() {
    let cases = [
        ("auto", EncoderChoice::Auto),
        ("AUTO", EncoderChoice::Auto),
        ("nvenc", EncoderChoice::Fixed(VideoEncoder::Nvenc)),
        (" NVENC ", EncoderChoice::Fixed(VideoEncoder::Nvenc)),
        ("Amf", EncoderChoice::Fixed(VideoEncoder::Amf)),
        ("qsv", EncoderChoice::Fixed(VideoEncoder::Qsv)),
        ("X264", EncoderChoice::Fixed(VideoEncoder::X264)),
    ];

    for (name, expected) in cases {
        let settings = settings_from(environment_with(&[("VIDEO_ENCODER", name)]))
            .expect("every encoder name is accepted");

        assert_eq!(settings.video_encoder, expected, "for {:?}", name);
    }
}
