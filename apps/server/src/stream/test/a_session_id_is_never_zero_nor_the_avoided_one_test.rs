use crate::stream::new_session_id;
use std::collections::HashSet;

#[test]
fn a_session_id_is_never_zero_nor_the_avoided_one() {
    let ids: Vec<u32> = (0..10_000).map(|_| new_session_id(7)).collect();
    let distinct: HashSet<u32> = ids.iter().copied().collect();

    assert!(ids.iter().all(|&id| id != 0 && id != 7));
    assert!(
        distinct.len() >= 9_990,
        "only {} distinct ids",
        distinct.len()
    );
}
