use super::environment_with;
use crate::settings::settings_from;

#[test]
fn an_unknown_video_encoder_is_refused_naming_video_encoder() {
    let refusal = settings_from(environment_with(&[("VIDEO_ENCODER", "vp9")]))
        .expect_err("vp9 is not one of the encoders");

    assert_eq!(refusal.variable, "VIDEO_ENCODER");
    assert_eq!(refusal.got, "vp9");
    assert!(
        refusal.expected.contains("nvenc"),
        "got {}",
        refusal.expected
    );
}
