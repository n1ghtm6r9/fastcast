pub fn probe_of_length(length: usize) -> Vec<u8> {
    let head = r#"{"type":"discover","version":1,"padding":""#;
    let tail = r#""}"#;
    let padding = "x".repeat(length - head.len() - tail.len());

    format!("{head}{padding}{tail}").into_bytes()
}
