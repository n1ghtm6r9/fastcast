use crate::fec::recover_data;
use crate::fec::test::{damaged_block, encoded_block};

#[test]
fn losing_more_shards_than_the_parity_count_cannot_be_recovered() {
    let original = encoded_block(5, 2, 8, 13);
    let (mut damaged, present) = damaged_block(&original, 8, &[0, 1, 2]);
    let before = damaged.clone();

    assert_eq!(recover_data(5, 2, 8, &mut damaged, &present), None);
    assert_eq!(damaged, before);
}
