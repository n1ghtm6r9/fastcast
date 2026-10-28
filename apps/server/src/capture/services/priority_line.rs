pub fn priority_line(outcomes: &[(&str, Result<(), String>)]) -> String {
    let parts: Vec<String> = outcomes
        .iter()
        .map(|(name, outcome)| match outcome {
            Ok(()) => (*name).to_owned(),
            Err(error) => format!("{name} failed: {error}"),
        })
        .collect();

    format!("capture priorities: {}", parts.join(", "))
}
