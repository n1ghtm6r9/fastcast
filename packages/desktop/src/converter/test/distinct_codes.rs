use std::collections::HashSet;

pub fn distinct_codes(samples: &[u16]) -> usize {
    samples.iter().collect::<HashSet<_>>().len()
}
