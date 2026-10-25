pub fn non_blank_value(lookup: &impl Fn(&str) -> Option<String>, variable: &str) -> Option<String> {
    lookup(variable)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
