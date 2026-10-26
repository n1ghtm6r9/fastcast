use crate::fixtures::TemporaryDirectory;
use crate::settings::{FrameRate, SavedSettings, read_saved_settings, write_saved_settings};
use std::fs;

#[test]
fn writing_settings_replaces_the_previous_file_and_leaves_nothing_else() {
    let directory = TemporaryDirectory::new("replaced-settings");
    let path = directory.path().join("settings.json");
    let first = SavedSettings {
        frame_rate: FrameRate::Fps30,
        ..SavedSettings::default()
    };
    let second = SavedSettings {
        frame_rate: FrameRate::Fps120,
        ..SavedSettings::default()
    };

    write_saved_settings(&path, &first).expect("the first settings are written");
    write_saved_settings(&path, &second).expect("the second settings are written");

    let left: Vec<_> = fs::read_dir(directory.path())
        .expect("the directory is listed")
        .map(|entry| entry.expect("the entry is read").file_name())
        .collect();
    assert_eq!(left, ["settings.json"]);
    assert_eq!(
        read_saved_settings(&path).expect("the settings are read"),
        second
    );
}
