use std::collections::HashMap;

pub fn environment_with(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let variables: HashMap<String, String> = pairs
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();

    move |name| variables.get(name).cloned()
}
