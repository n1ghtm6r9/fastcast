pub fn profile_space_prefix(profile_space: u8) -> &'static str {
    match profile_space {
        1 => "A",
        2 => "B",
        3 => "C",
        _ => "",
    }
}
