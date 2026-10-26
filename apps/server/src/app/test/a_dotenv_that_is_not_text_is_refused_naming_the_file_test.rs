use crate::app::{DotenvError, load_dotenv_file};
use crate::fixtures::TemporaryDirectory;

#[test]
fn a_dotenv_that_is_not_text_is_refused_naming_the_file() {
    let directory = TemporaryDirectory::new("a_dotenv_that_is_not_text");
    let path = directory.write(".env", b"SERVER_NAME=\xFF\xFE\n");

    let failure =
        load_dotenv_file(&path).expect_err("bytes that are not text must stop the server");

    assert!(
        matches!(&failure, DotenvError::NotText { path: named } if *named == path),
        "got {failure:?}"
    );
}
