use crate::annexb::{START_CODE, start_code_position, without_trailing_zeros};

pub fn annex_b_units(data: &[u8]) -> Vec<&[u8]> {
    let mut units = Vec::new();
    let mut start_code = start_code_position(data, 0);

    while let Some(position) = start_code {
        let unit_start = position + START_CODE.len();
        let next_start_code = start_code_position(data, unit_start);
        let unit_end = next_start_code.unwrap_or(data.len());
        let unit = without_trailing_zeros(&data[unit_start..unit_end]);

        if !unit.is_empty() {
            units.push(unit);
        }

        start_code = next_start_code;
    }

    units
}
