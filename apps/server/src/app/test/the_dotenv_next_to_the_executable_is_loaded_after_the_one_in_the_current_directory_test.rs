use crate::app::load_dotenv_files;
use crate::fixtures::TemporaryDirectory;
use std::env;

#[test]
fn the_dotenv_next_to_the_executable_is_loaded_after_the_one_in_the_current_directory() {
    let directory = TemporaryDirectory::new("two_dotenv_files");
    directory.write(
        "work/.env",
        b"FASTCAST_TEST_TWO_FIRST=work\nFASTCAST_TEST_TWO_SHARED=work\n",
    );
    directory.write(
        "bin/.env",
        b"FASTCAST_TEST_TWO_SECOND=bin\nFASTCAST_TEST_TWO_SHARED=bin\n",
    );

    load_dotenv_files(
        Some(&directory.path().join("work")),
        Some(&directory.path().join("bin")),
    )
    .expect("both files are valid");

    assert_eq!(env::var("FASTCAST_TEST_TWO_FIRST").as_deref(), Ok("work"));
    assert_eq!(env::var("FASTCAST_TEST_TWO_SECOND").as_deref(), Ok("bin"));
    assert_eq!(env::var("FASTCAST_TEST_TWO_SHARED").as_deref(), Ok("work"));
}
