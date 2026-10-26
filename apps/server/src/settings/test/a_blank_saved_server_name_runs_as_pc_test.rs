use crate::settings::{SavedSettings, settings_of};

#[test]
fn a_blank_saved_server_name_runs_as_pc() {
    let blank = SavedSettings {
        server_name: "   ".to_owned(),
        ..SavedSettings::default()
    };
    let named = SavedSettings {
        server_name: "  Gaming PC ".to_owned(),
        ..SavedSettings::default()
    };

    assert_eq!(settings_of(&blank).server_name, "PC");
    assert_eq!(settings_of(&named).server_name, "Gaming PC");
}
