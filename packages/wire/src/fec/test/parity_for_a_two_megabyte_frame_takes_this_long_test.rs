use crate::fec::encode_parity;
use crate::packet::SHARD_SIZE;
use crate::test_support::random_bytes;
use std::time::Instant;

#[test]
#[ignore = "benchmark"]
fn parity_for_a_two_megabyte_frame_takes_this_long() {
    let blocks = 16;
    let data_shards = 96;
    let parity_shards = 10;
    let mut state = 0x2545_F491_4F6C_DD1D;
    let mut frame: Vec<Vec<u8>> = (0..blocks)
        .map(|_| random_bytes(&mut state, (data_shards + parity_shards) * SHARD_SIZE))
        .collect();
    let started = Instant::now();

    for block in &mut frame {
        encode_parity(data_shards, parity_shards, SHARD_SIZE, block);
    }

    let elapsed = started.elapsed();
    let megabytes = (blocks * data_shards * SHARD_SIZE) as f64 / 1_000_000.0;
    println!(
        "parity for {megabytes:.2} MB of data took {elapsed:?}, {:.0} MB/s",
        megabytes / elapsed.as_secs_f64()
    );
}
