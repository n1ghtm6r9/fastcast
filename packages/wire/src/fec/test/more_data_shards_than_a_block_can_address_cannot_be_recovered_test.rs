use crate::fec::{MAX_DATA_SHARDS, recover_data};

#[test]
fn more_data_shards_than_a_block_can_address_cannot_be_recovered() {
    let data_shards = MAX_DATA_SHARDS + 1;
    let mut block = vec![7u8; data_shards + 1];
    let mut present = vec![true; data_shards + 1];
    present[0] = false;

    assert_eq!(recover_data(data_shards, 1, 1, &mut block, &present), None);
    assert!(block.iter().all(|&byte| byte == 7));
}
