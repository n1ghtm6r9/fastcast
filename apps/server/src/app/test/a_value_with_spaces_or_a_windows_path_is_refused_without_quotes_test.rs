use crate::app::{DotenvError, load_dotenv_file};
use crate::fixtures::TemporaryDirectory;

#[test]
fn a_value_with_spaces_or_a_windows_path_is_refused_without_quotes() {
    let directory = TemporaryDirectory::new("a_value_without_quotes");
    let lines = [
        "SERVER_NAME=Gaming PC",
        r"FFMPEG_PATH=C:\Tools\ffmpeg\bin\ffmpeg.exe",
    ];

    for line in lines {
        let path = directory.write(".env", format!("{line}\n").as_bytes());

        let failure =
            load_dotenv_file(&path).expect_err("an unquoted value like this cannot be parsed");

        assert!(
            matches!(&failure, DotenvError::Line { number: 1, text, .. } if text == line),
            "for {line}: got {failure:?}"
        );
    }
}
