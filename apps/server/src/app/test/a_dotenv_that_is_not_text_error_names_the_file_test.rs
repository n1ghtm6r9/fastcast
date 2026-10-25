use crate::app::DotenvError;
use std::path::PathBuf;

#[test]
fn a_dotenv_that_is_not_text_error_names_the_file() {
    let failure = DotenvError::NotText {
        path: PathBuf::from(r"C:\Tools\fastcast\.env"),
    };

    assert_eq!(
        failure.to_string(),
        r"C:\Tools\fastcast\.env is not UTF-8 or UTF-16 text, save it as UTF-8"
    );
}
