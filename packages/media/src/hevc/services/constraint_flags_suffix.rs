pub fn constraint_flags_suffix(constraint_flags: &[u8]) -> String {
    let used = constraint_flags
        .iter()
        .rposition(|&byte| byte != 0)
        .map_or(0, |last| last + 1);

    constraint_flags[..used]
        .iter()
        .map(|byte| format!(".{:02X}", byte))
        .collect()
}
