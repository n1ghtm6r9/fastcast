use crate::app::{DotenvError, load_dotenv_file};
use crate::fixtures::TemporaryDirectory;

#[test]
fn a_line_dotenvy_cannot_parse_is_refused_naming_the_file_and_the_line() {
    let directory = TemporaryDirectory::new("a_line_dotenvy_cannot_parse");
    let path = directory.write(
        ".env",
        b"FASTCAST_TEST_ABOVE=1\r\n# a comment\r\n\r\nSERVER_NAME=Gaming PC\r\nFASTCAST_TEST_BELOW=2\r\n",
    );

    let failure =
        load_dotenv_file(&path).expect_err("a line that cannot be parsed must stop the server");

    assert!(
        matches!(
            &failure,
            DotenvError::Line { path: named, number: 4, text }
                if *named == path && text == "SERVER_NAME=Gaming PC"
        ),
        "got {failure:?}"
    );
}
