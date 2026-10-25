use crate::fec::recover_data;
use crate::fec::test::encoded_block;

#[test]
fn a_block_with_every_data_shard_present_needs_no_recovery() {
    let mut block = encoded_block(4, 2, 8, 11);
    let before = block.clone();
    let present = [true, true, true, true, false, false];

    assert_eq!(recover_data(4, 2, 8, &mut block, &present), Some(0));
    assert_eq!(block, before);
}
