use crate::fec::encode_parity;
use crate::gf::{gf_inv, gf_mul};

#[test]
fn one_parity_shard_of_one_data_shard_is_the_data_times_its_coefficient() {
    let mut block = vec![1, 2, 3, 4, 0, 0, 0, 0];

    encode_parity(1, 1, 4, &mut block);

    assert_eq!(
        block[4..],
        [1, 2, 3, 4].map(|byte| gf_mul(gf_inv(128), byte))
    );
}
