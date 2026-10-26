use crate::settings::{FrameRate, SavedSettings};

#[test]
fn a_saved_file_without_some_settings_takes_their_defaults() {
    let saved: SavedSettings =
        serde_json::from_str(r#"{"frame_rate":"120"}"#).expect("the settings parse");

    assert_eq!(
        saved,
        SavedSettings {
            frame_rate: FrameRate::Fps120,
            ..SavedSettings::default()
        }
    );
}
