use crate::settings::{SavedSettings, settings_of};

#[test]
fn a_blank_saved_server_name_runs_as_the_computer_name() {
    let blank = SavedSettings {
        server_name: "   ".to_owned(),
        ..SavedSettings::default()
    };
    let named = SavedSettings {
        server_name: "  Gaming PC ".to_owned(),
        ..SavedSettings::default()
    };

    assert_eq!(settings_of(&blank, "DESKTOP-1").server_name, "DESKTOP-1");
    assert_eq!(settings_of(&named, "DESKTOP-1").server_name, "Gaming PC");
}
