use crate::settings::{FrameRate, SavedSettings};

#[test]
fn a_frame_rate_is_saved_as_its_number_of_frames() {
    let saved = SavedSettings {
        frame_rate: FrameRate::Fps24,
        ..SavedSettings::default()
    };

    let json = serde_json::to_value(&saved).expect("the settings serialize");

    assert_eq!(json["frame_rate"], "24");
}
