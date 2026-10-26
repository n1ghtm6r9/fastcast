use crate::app::{DotenvError, load_dotenv_file};
use crate::fixtures::TemporaryDirectory;
use std::fs;

#[test]
fn an_unreadable_dotenv_is_refused_naming_the_file_and_the_cause() {
    let directory = TemporaryDirectory::new("an_unreadable_dotenv");
    let path = directory.path().join(".env");
    fs::create_dir(&path).expect("a directory can stand where the file should be");

    let failure =
        load_dotenv_file(&path).expect_err("a file that cannot be read must stop the server");

    assert!(
        matches!(&failure, DotenvError::Read { path: named, .. } if *named == path),
        "got {failure:?}"
    );
}
