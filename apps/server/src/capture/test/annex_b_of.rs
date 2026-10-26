pub fn annex_b_of(units: &[&[u8]]) -> Vec<u8> {
    let mut annex_b = Vec::new();

    for unit in units {
        annex_b.extend_from_slice(&[0, 0, 0, 1]);
        annex_b.extend_from_slice(unit);
    }

    annex_b
}
