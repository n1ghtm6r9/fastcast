use crate::settings::SavedSettings;

#[test]
fn saved_numbers_outside_their_limits_are_brought_within_them() {
    let above = SavedSettings {
        capture_output: 99,
        video_bitrate_mbps: 900,
        ..SavedSettings::default()
    };
    let below = SavedSettings {
        video_bitrate_mbps: 0,
        ..SavedSettings::default()
    };

    assert_eq!(
        above.within_limits(),
        SavedSettings {
            capture_output: 63,
            video_bitrate_mbps: 200,
            ..SavedSettings::default()
        }
    );
    assert_eq!(below.within_limits().video_bitrate_mbps, 1);
}
