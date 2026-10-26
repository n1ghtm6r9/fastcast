pub fn trimmed_or(text: &str, fallback: &str) -> String {
    match text.trim() {
        "" => fallback.to_owned(),
        trimmed => trimmed.to_owned(),
    }
}
