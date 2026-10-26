pub fn random_payload(seed: u64, length: usize) -> Vec<u8> {
    let mut state = seed.max(1);

    (0..length)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;

            (state >> 32) as u8
        })
        .collect()
}
