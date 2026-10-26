use crate::settings::SavedSettings;

#[test]
fn a_saved_file_without_some_settings_takes_their_defaults() {
    let saved: SavedSettings =
        serde_json::from_str(r#"{"capture_fps":120}"#).expect("the settings parse");

    assert_eq!(
        saved,
        SavedSettings {
            capture_fps: 120,
            ..SavedSettings::default()
        }
    );
}
