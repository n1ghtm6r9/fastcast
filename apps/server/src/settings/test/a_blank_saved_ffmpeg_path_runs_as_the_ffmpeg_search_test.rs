use crate::settings::constants::FFMPEG_SEARCH_PATH;
use crate::settings::{SavedSettings, settings_of};

#[test]
fn a_blank_saved_ffmpeg_path_runs_as_the_ffmpeg_search() {
    let chosen = SavedSettings {
        ffmpeg_path: r" C:\Tools\ffmpeg.exe ".to_owned(),
        ..SavedSettings::default()
    };

    assert_eq!(
        settings_of(&SavedSettings::default(), "PC").ffmpeg_path,
        FFMPEG_SEARCH_PATH
    );
    assert_eq!(
        settings_of(&chosen, "PC").ffmpeg_path,
        r"C:\Tools\ffmpeg.exe"
    );
}
