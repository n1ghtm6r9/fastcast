use crate::app::{DotenvError, load_dotenv_files};
use crate::fixtures::TemporaryDirectory;
use std::env;

#[test]
fn a_dotenv_that_cannot_be_parsed_stops_the_loading_of_the_next_one() {
    let directory = TemporaryDirectory::new("a_broken_dotenv_first");
    let broken = directory.write("work/.env", b"SERVER_NAME=Gaming PC\n");
    directory.write("bin/.env", b"FASTCAST_TEST_NEVER_LOADED=bin\n");

    let failure = load_dotenv_files(
        Some(&directory.path().join("work")),
        Some(&directory.path().join("bin")),
    )
    .expect_err("the broken file must stop the server");

    assert!(
        matches!(&failure, DotenvError::Line { path, .. } if *path == broken),
        "got {failure:?}"
    );
    assert!(env::var("FASTCAST_TEST_NEVER_LOADED").is_err());
}
