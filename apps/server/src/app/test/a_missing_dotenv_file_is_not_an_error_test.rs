use crate::app::load_dotenv_file;
use crate::fixtures::TemporaryDirectory;

#[test]
fn a_missing_dotenv_file_is_not_an_error() {
    let directory = TemporaryDirectory::new("a_missing_dotenv_file");

    let outcome = load_dotenv_file(&directory.path().join(".env"));

    assert!(outcome.is_ok());
}
