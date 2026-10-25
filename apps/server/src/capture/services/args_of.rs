pub fn args_of(line: &str) -> Vec<String> {
    line.split_whitespace().map(str::to_owned).collect()
}
