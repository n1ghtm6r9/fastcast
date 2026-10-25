use crate::avc::BitReader;

const SCALE_MODULUS: i64 = 256;

pub fn skip_scaling_list(reader: &mut BitReader, size: usize) -> Option<()> {
    let mut last_scale: i64 = 8;
    let mut next_scale: i64 = 8;

    for _ in 0..size {
        if next_scale != 0 {
            let delta_scale = reader.read_se()?;

            next_scale = (last_scale + delta_scale + SCALE_MODULUS).rem_euclid(SCALE_MODULUS);
        }

        if next_scale != 0 {
            last_scale = next_scale;
        }
    }

    Some(())
}
