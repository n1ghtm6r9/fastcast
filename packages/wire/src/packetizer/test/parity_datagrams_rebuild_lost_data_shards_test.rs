use crate::fec::recover_data;
use crate::packet::SHARD_SIZE;
use crate::packetizer::test::{shard_of, video_frame};
use crate::packetizer::{Datagrams, Packetizer};
use crate::test_support::random_bytes;

#[test]
fn parity_datagrams_rebuild_lost_data_shards() {
    let mut state = 9;
    let payload = random_bytes(&mut state, 3000);
    let mut packetizer = Packetizer::new(1);
    let mut out = Datagrams::new();
    let mut block = vec![0u8; 5 * SHARD_SIZE];
    let mut present = [false; 5];

    packetizer.frame(&video_frame(&payload, false), 50, &mut out);

    for datagram in out.iter() {
        let shard = shard_of(datagram);
        let index = shard.header.shard as usize;

        if index == 0 || index == 2 {
            continue;
        }

        block[index * SHARD_SIZE..index * SHARD_SIZE + shard.data.len()]
            .copy_from_slice(shard.data);
        present[index] = true;
    }

    assert_eq!(
        recover_data(3, 2, SHARD_SIZE, &mut block, &present),
        Some(2)
    );
    assert_eq!(block[..SHARD_SIZE], payload[..SHARD_SIZE]);
    assert_eq!(
        block[2 * SHARD_SIZE..2 * SHARD_SIZE + 260],
        payload[2 * SHARD_SIZE..]
    );
}
