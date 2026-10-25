use crate::fec::encode_parity;

#[test]
fn parity_of_an_all_zero_block_is_zero() {
    let shard_size = 16;
    let mut block = vec![0u8; 8 * shard_size];
    block[5 * shard_size..].fill(0xAA);

    encode_parity(5, 3, shard_size, &mut block);

    assert!(block.iter().all(|&byte| byte == 0));
}
