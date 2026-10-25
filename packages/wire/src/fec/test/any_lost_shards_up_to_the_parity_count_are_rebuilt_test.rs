use crate::fec::recover_data;
use crate::fec::test::{damaged_block, encoded_block};

#[test]
fn any_lost_shards_up_to_the_parity_count_are_rebuilt() {
    let data_shards = 10;
    let parity_shards = 4;
    let shard_size = 64;
    let total_shards = data_shards + parity_shards;
    let data_length = data_shards * shard_size;
    let original = encoded_block(data_shards, parity_shards, shard_size, 0x9E37_79B9);

    for lost in 1u32..(1 << total_shards) {
        if lost.count_ones() as usize > parity_shards {
            continue;
        }

        let absent: Vec<usize> = (0..total_shards)
            .filter(|shard| (lost >> shard) & 1 == 1)
            .collect();
        let absent_data = absent.iter().filter(|&&shard| shard < data_shards).count();
        let (mut damaged, present) = damaged_block(&original, shard_size, &absent);

        assert_eq!(
            recover_data(
                data_shards,
                parity_shards,
                shard_size,
                &mut damaged,
                &present
            ),
            Some(absent_data)
        );
        assert_eq!(damaged[..data_length], original[..data_length]);
    }
}
