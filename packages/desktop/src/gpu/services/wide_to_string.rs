pub fn wide_to_string(wide: &[u16]) -> String {
    let length = wide
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(wide.len());

    String::from_utf16_lossy(&wide[..length])
}
