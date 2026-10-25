use crate::test_support::xorshift;

pub fn random_bytes(state: &mut u64, length: usize) -> Vec<u8> {
    (0..length).map(|_| (xorshift(state) >> 24) as u8).collect()
}
