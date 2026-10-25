#[test]
fn the_documented_single_quoted_values_are_read_as_written() {
    let text = [
        "SERVER_NAME='Gaming PC'",
        r"FFMPEG_PATH='C:\Tools\ffmpeg\bin\ffmpeg.exe'",
        r"FFMPEG_PATH_WITH_SPACES='C:\Program Files\ffmpeg\bin\ffmpeg.exe'",
        r"FFMPEG_PATH_WITH_ESCAPE_LOOKALIKES='C:\new\tools\ffmpeg.exe'",
    ]
    .join("\n");

    let pairs: Vec<(String, String)> = dotenvy::from_read_iter(text.as_bytes())
        .collect::<Result<_, _>>()
        .expect("single quotes keep spaces and backslashes as they are");

    assert_eq!(
        pairs,
        vec![
            ("SERVER_NAME".to_owned(), "Gaming PC".to_owned()),
            (
                "FFMPEG_PATH".to_owned(),
                r"C:\Tools\ffmpeg\bin\ffmpeg.exe".to_owned()
            ),
            (
                "FFMPEG_PATH_WITH_SPACES".to_owned(),
                r"C:\Program Files\ffmpeg\bin\ffmpeg.exe".to_owned()
            ),
            (
                "FFMPEG_PATH_WITH_ESCAPE_LOOKALIKES".to_owned(),
                r"C:\new\tools\ffmpeg.exe".to_owned()
            ),
        ]
    );
}
