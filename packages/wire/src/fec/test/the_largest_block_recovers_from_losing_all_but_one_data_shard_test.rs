use crate::fec::test::{damaged_block, encoded_block};
use crate::fec::{MAX_DATA_SHARDS, MAX_PARITY_SHARDS, recover_data};

#[test]
fn the_largest_block_recovers_from_losing_all_but_one_data_shard() {
    let shard_size = 16;
    let data_length = MAX_DATA_SHARDS * shard_size;
    let original = encoded_block(MAX_DATA_SHARDS, MAX_PARITY_SHARDS, shard_size, 17);
    let absent: Vec<usize> = (0..MAX_DATA_SHARDS - 1).collect();
    let (mut damaged, present) = damaged_block(&original, shard_size, &absent);

    assert_eq!(
        recover_data(
            MAX_DATA_SHARDS,
            MAX_PARITY_SHARDS,
            shard_size,
            &mut damaged,
            &present
        ),
        Some(127)
    );
    assert_eq!(damaged[..data_length], original[..data_length]);
}
