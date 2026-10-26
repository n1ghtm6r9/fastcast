use crate::app::data_directory_from;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

#[test]
fn the_data_directory_is_fastcast_in_appdata_or_in_the_temporary_directory() {
    let roaming = |variable: &str| {
        (variable == "APPDATA").then(|| OsString::from(r"C:\Users\me\AppData\Roaming"))
    };

    assert_eq!(
        data_directory_from(roaming),
        PathBuf::from(r"C:\Users\me\AppData\Roaming\fastcast")
    );
    assert_eq!(
        data_directory_from(|_| None),
        env::temp_dir().join("fastcast")
    );
    assert_eq!(
        data_directory_from(|_| Some(OsString::new())),
        env::temp_dir().join("fastcast")
    );
}
