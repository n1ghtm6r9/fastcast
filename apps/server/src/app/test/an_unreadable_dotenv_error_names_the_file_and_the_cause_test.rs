use crate::app::DotenvError;
use std::io;
use std::path::PathBuf;

#[test]
fn an_unreadable_dotenv_error_names_the_file_and_the_cause() {
    let failure = DotenvError::Read {
        path: PathBuf::from(r"C:\Tools\fastcast\.env"),
        source: io::Error::other("boom"),
    };

    assert_eq!(
        failure.to_string(),
        r"could not read C:\Tools\fastcast\.env: boom"
    );
}
