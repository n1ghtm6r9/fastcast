use crate::app::DotenvError;
use std::path::PathBuf;

#[test]
fn a_dotenv_line_error_names_the_file_the_line_and_the_text() {
    let failure = DotenvError::Line {
        path: PathBuf::from(r"C:\Tools\fastcast\.env"),
        number: 3,
        text: "SERVER_NAME=Gaming PC".to_owned(),
    };

    assert_eq!(
        failure.to_string(),
        "C:\\Tools\\fastcast\\.env line 3 cannot be parsed: \"SERVER_NAME=Gaming PC\" \
         (write NAME=value, and put a value with spaces or backslashes in single quotes)"
    );
}
