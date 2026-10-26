use crate::settings::SavedSettings;

#[test]
fn saved_numbers_outside_their_limits_are_brought_within_them() {
    let saved = SavedSettings {
        capture_output: 99,
        capture_fps: 0,
        video_bitrate_kbps: 900_000,
        keyframe_interval_seconds: 0,
        ..SavedSettings::default()
    };

    assert_eq!(
        saved.within_limits(),
        SavedSettings {
            capture_output: 63,
            capture_fps: 1,
            video_bitrate_kbps: 200_000,
            keyframe_interval_seconds: 1,
            ..SavedSettings::default()
        }
    );
}
