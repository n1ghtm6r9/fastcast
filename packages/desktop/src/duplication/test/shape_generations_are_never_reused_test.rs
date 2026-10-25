use crate::duplication::next_shape_generation;
use std::collections::HashSet;

#[test]
fn shape_generations_are_never_reused() {
    let generations: HashSet<u64> = (0..1000).map(|_| next_shape_generation()).collect();

    assert_eq!(generations.len(), 1000);
}
