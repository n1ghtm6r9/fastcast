pub fn without_trailing_zeros(unit: &[u8]) -> &[u8] {
    let length = unit
        .iter()
        .rposition(|&byte| byte != 0)
        .map_or(0, |last| last + 1);

    &unit[..length]
}
