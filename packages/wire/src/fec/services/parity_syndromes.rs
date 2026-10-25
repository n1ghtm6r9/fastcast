use crate::fec::cauchy_coefficient;
use crate::gf::mul_add_slice;

pub fn parity_syndromes(
    data_shards: usize,
    shard_size: usize,
    block: &[u8],
    present: &[bool],
    rows: &[usize],
) -> Vec<u8> {
    let mut syndromes = vec![0u8; rows.len() * shard_size];

    for (index, &row) in rows.iter().enumerate() {
        let syndrome = &mut syndromes[index * shard_size..(index + 1) * shard_size];
        let parity_start = (data_shards + row) * shard_size;

        syndrome.copy_from_slice(&block[parity_start..parity_start + shard_size]);

        for column in (0..data_shards).filter(|&column| present[column]) {
            let data_shard = &block[column * shard_size..(column + 1) * shard_size];
            mul_add_slice(cauchy_coefficient(row, column), data_shard, syndrome);
        }
    }

    syndromes
}
