pub fn nal_units(annex_b: &[u8]) -> Vec<&[u8]> {
    let starts: Vec<usize> = annex_b
        .windows(3)
        .enumerate()
        .filter(|(_, window)| *window == [0, 0, 1])
        .map(|(index, _)| index + 3)
        .collect();

    starts
        .iter()
        .enumerate()
        .map(|(position, start)| {
            let end = starts
                .get(position + 1)
                .map_or(annex_b.len(), |next| next - 3);
            &annex_b[*start..end]
        })
        .filter(|unit| !unit.is_empty())
        .collect()
}
