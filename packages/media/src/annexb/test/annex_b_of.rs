use crate::annexb::START_CODE;

pub fn annex_b_of(units: &[&[u8]]) -> Vec<u8> {
    let mut annex_b = Vec::new();

    for unit in units {
        annex_b.push(0);
        annex_b.extend_from_slice(&START_CODE);
        annex_b.extend_from_slice(unit);
    }

    annex_b
}
