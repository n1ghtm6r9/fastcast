use crate::settings::SavedSettings;
use serde_json::Value;

#[test]
fn the_debug_box_is_saved_as_debug() {
    let json = serde_json::to_value(SavedSettings {
        debug: true,
        ..SavedSettings::default()
    })
    .expect("the settings serialize");

    assert_eq!(json["debug"], Value::Bool(true));
}
