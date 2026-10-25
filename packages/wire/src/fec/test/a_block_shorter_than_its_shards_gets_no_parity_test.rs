use crate::fec::encode_parity;

#[test]
fn a_block_shorter_than_its_shards_gets_no_parity() {
    let mut block = vec![0xFFu8; 3 * 8 + 2 * 8 - 1];

    encode_parity(3, 2, 8, &mut block);

    assert!(block.iter().all(|&byte| byte == 0xFF));
}
