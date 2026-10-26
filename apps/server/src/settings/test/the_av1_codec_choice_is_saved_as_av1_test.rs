use crate::settings::{CodecChoice, SavedSettings};

#[test]
fn the_av1_codec_choice_is_saved_as_av1() {
    let saved = SavedSettings {
        video_codec: CodecChoice::Av1,
        ..SavedSettings::default()
    };

    let json = serde_json::to_value(&saved).expect("the settings serialize");

    assert_eq!(json["video_codec"], "av1");
}
