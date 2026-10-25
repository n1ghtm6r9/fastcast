use crate::layout::parity_shards_for;

#[test]
fn parity_is_the_rounded_up_percentage_of_the_data_shards() {
    assert_eq!(parity_shards_for(10, 10), 1);
    assert_eq!(parity_shards_for(11, 10), 2);
    assert_eq!(parity_shards_for(1, 10), 1);
    assert_eq!(parity_shards_for(128, 50), 64);
    assert_eq!(parity_shards_for(128, 100), 127);
    assert_eq!(parity_shards_for(5, 0), 0);
}
