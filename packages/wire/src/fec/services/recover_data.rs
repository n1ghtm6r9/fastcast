use crate::fec::{block_length, cauchy_coefficient, invert_matrix, parity_syndromes};
use crate::gf::mul_add_slice;

pub fn recover_data(
    data_shards: usize,
    parity_shards: usize,
    shard_size: usize,
    block: &mut [u8],
    present: &[bool],
) -> Option<usize> {
    let length = block_length(data_shards, parity_shards, shard_size)?;

    if block.len() < length || present.len() != data_shards + parity_shards {
        return None;
    }

    let absent: Vec<usize> = (0..data_shards).filter(|&shard| !present[shard]).collect();

    if absent.is_empty() {
        return Some(0);
    }

    let rows: Vec<usize> = (0..parity_shards)
        .filter(|&row| present[data_shards + row])
        .take(absent.len())
        .collect();

    if rows.len() < absent.len() {
        return None;
    }

    let matrix: Vec<u8> = rows
        .iter()
        .flat_map(|&row| {
            absent
                .iter()
                .map(move |&column| cauchy_coefficient(row, column))
        })
        .collect();
    let inverse = invert_matrix(&matrix, absent.len())?;
    let syndromes = parity_syndromes(data_shards, shard_size, block, present, &rows);

    for (index, &shard) in absent.iter().enumerate() {
        let target = &mut block[shard * shard_size..(shard + 1) * shard_size];

        target.fill(0);

        for row in 0..rows.len() {
            let syndrome = &syndromes[row * shard_size..(row + 1) * shard_size];
            mul_add_slice(inverse[index * rows.len() + row], syndrome, target);
        }
    }

    Some(absent.len())
}
