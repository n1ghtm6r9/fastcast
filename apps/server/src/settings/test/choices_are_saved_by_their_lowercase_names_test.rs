use crate::settings::{BackendChoice, EncoderChoice, LogLevel, SavedSettings, VideoEncoder};

#[test]
fn choices_are_saved_by_their_lowercase_names() {
    let saved = SavedSettings {
        capture_backend: BackendChoice::Native,
        video_encoder: EncoderChoice::Fixed(VideoEncoder::X264),
        log_level: LogLevel::Debug,
        ..SavedSettings::default()
    };

    let json = serde_json::to_value(&saved).expect("the settings serialize");

    assert_eq!(json["capture_backend"], "native");
    assert_eq!(json["video_encoder"], "x264");
    assert_eq!(json["log_level"], "debug");
}
