use std::time::Duration;

pub fn nearest_rank(sorted: &[Duration], percent: usize) -> Option<Duration> {
    let rank = (sorted.len() * percent).div_ceil(100);

    sorted.get(rank.saturating_sub(1)).copied()
}
