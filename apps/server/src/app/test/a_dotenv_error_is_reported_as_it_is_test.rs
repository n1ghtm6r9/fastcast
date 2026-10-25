use crate::app::{AppError, DotenvError};
use std::path::PathBuf;

#[test]
fn a_dotenv_error_is_reported_as_it_is() {
    let failure = AppError::from(DotenvError::NotText {
        path: PathBuf::from(r"C:\Tools\fastcast\.env"),
    });

    assert_eq!(
        failure.to_string(),
        r"C:\Tools\fastcast\.env is not UTF-8 or UTF-16 text, save it as UTF-8"
    );
}
