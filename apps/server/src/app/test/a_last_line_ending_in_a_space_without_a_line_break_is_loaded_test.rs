use super::TemporaryDirectory;
use crate::app::load_dotenv_file;
use std::env;

#[test]
fn a_last_line_ending_in_a_space_without_a_line_break_is_loaded() {
    let directory = TemporaryDirectory::new("a_last_line_ending_in_a_space");
    let path = directory.write(".env", b"FASTCAST_TEST_LAST_LINE=info ");

    load_dotenv_file(&path).expect("a trailing space is not a mistake");

    assert_eq!(env::var("FASTCAST_TEST_LAST_LINE").as_deref(), Ok("info"));
}
