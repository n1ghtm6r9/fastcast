use crate::app::window_problem_text;
use std::path::Path;

#[test]
fn a_window_that_cannot_open_is_explained_with_the_journal_file() {
    let text = window_problem_text(
        "OpenGL 2.0 is not supported",
        Path::new(r"C:\Users\me\AppData\Roaming\fastcast\fastcast.log"),
    );

    assert_eq!(
        text,
        "FastCast не смог открыть окно: OpenGL 2.0 is not supported\n\n\
         Подробности в журнале: C:\\Users\\me\\AppData\\Roaming\\fastcast\\fastcast.log"
    );
}
