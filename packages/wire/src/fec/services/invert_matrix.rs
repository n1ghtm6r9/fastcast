use crate::gf::{gf_inv, gf_mul, mul_add_slice};

pub fn invert_matrix(matrix: &[u8], size: usize) -> Option<Vec<u8>> {
    let width = size * 2;
    let mut work = vec![0u8; size * width];

    for row in 0..size {
        let start = row * width;
        work[start..start + size].copy_from_slice(&matrix[row * size..(row + 1) * size]);
        work[start + size + row] = 1;
    }

    for column in 0..size {
        let pivot_row = (column..size).find(|&row| work[row * width + column] != 0)?;

        for offset in 0..width {
            work.swap(column * width + offset, pivot_row * width + offset);
        }

        let scale = gf_inv(work[column * width + column]);

        for value in &mut work[column * width..(column + 1) * width] {
            *value = gf_mul(scale, *value);
        }

        let pivot = work[column * width..(column + 1) * width].to_vec();

        for row in (0..size).filter(|&row| row != column) {
            let factor = work[row * width + column];
            mul_add_slice(factor, &pivot, &mut work[row * width..(row + 1) * width]);
        }
    }

    Some(
        (0..size)
            .flat_map(|row| work[row * width + size..(row + 1) * width].iter().copied())
            .collect(),
    )
}
