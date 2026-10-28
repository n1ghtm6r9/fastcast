use crate::stream::{MEDIAN_PERCENT, MOST_PERCENT, milliseconds_of, nearest_rank};
use std::time::Duration;

pub fn duration_spread(durations: &mut [Duration]) -> String {
    durations.sort_unstable();

    let (Some(median), Some(most), Some(longest)) = (
        nearest_rank(durations, MEDIAN_PERCENT),
        nearest_rank(durations, MOST_PERCENT),
        durations.last(),
    ) else {
        return "n/a".to_owned();
    };

    format!(
        "{:.1} ms, {MOST_PERCENT}% within {:.1} ms, longest {:.1} ms",
        milliseconds_of(median),
        milliseconds_of(most),
        milliseconds_of(*longest)
    )
}
