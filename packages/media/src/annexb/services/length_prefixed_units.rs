pub fn length_prefixed_units(units: &[&[u8]]) -> Vec<u8> {
    let total = units.iter().map(|unit| size_of::<u32>() + unit.len()).sum();
    let mut output = Vec::with_capacity(total);

    for unit in units {
        let length = u32::try_from(unit.len()).expect("a unit is shorter than 4 GiB");

        output.extend_from_slice(&length.to_be_bytes());
        output.extend_from_slice(unit);
    }

    output
}
