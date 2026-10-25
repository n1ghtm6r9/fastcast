use crate::colour::BitDepth;

pub fn plane_samples(
    mapped: &[u8],
    offset: usize,
    row_pitch: usize,
    samples_per_row: usize,
    rows: usize,
    depth: BitDepth,
) -> Vec<u16> {
    let mut samples = Vec::with_capacity(samples_per_row * rows);

    for row in 0..rows {
        let start = offset + row * row_pitch;

        match depth {
            BitDepth::Eight => samples.extend(
                mapped[start..start + samples_per_row]
                    .iter()
                    .map(|sample| u16::from(*sample)),
            ),
            BitDepth::Ten => samples.extend(
                mapped[start..start + samples_per_row * 2]
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|pair| u16::from_le_bytes(*pair) >> 6),
            ),
        }
    }

    samples
}
