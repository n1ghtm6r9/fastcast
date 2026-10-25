pub fn damaged_block(block: &[u8], shard_size: usize, absent: &[usize]) -> (Vec<u8>, Vec<bool>) {
    let mut damaged = block.to_vec();
    let mut present = vec![true; block.len() / shard_size];

    for &shard in absent {
        damaged[shard * shard_size..(shard + 1) * shard_size].fill(0);
        present[shard] = false;
    }

    (damaged, present)
}
