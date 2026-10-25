pub fn is_parsable_dotenv(text: &str) -> bool {
    dotenvy::from_read_iter(text.as_bytes()).all(|entry| entry.is_ok())
}
