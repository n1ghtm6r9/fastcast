use crate::settings::{CodecChoice, FrameRate, LogLevel, SavedSettings};

#[test]
fn choices_are_saved_by_their_lowercase_names() {
    let saved = SavedSettings {
        frame_rate: FrameRate::Unlimited,
        video_codec: CodecChoice::Hevc,
        log_level: LogLevel::Debug,
        ..SavedSettings::default()
    };

    let json = serde_json::to_value(&saved).expect("the settings serialize");

    assert_eq!(json["frame_rate"], "unlimited");
    assert_eq!(json["video_codec"], "hevc");
    assert_eq!(json["log_level"], "debug");
}
