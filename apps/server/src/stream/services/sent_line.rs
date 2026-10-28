use crate::stream::{MEDIAN_PERCENT, MOST_PERCENT, milliseconds_of, nearest_rank};
use std::time::Duration;

pub fn sent_line(delays: &mut [Duration]) -> String {
    delays.sort_unstable();

    let (Some(median), Some(most)) = (
        nearest_rank(delays, MEDIAN_PERCENT),
        nearest_rank(delays, MOST_PERCENT),
    ) else {
        return String::new();
    };

    format!(
        "; the PC sent frames {:.1} ms after the screen showed them, {MOST_PERCENT}% within {:.1} ms",
        milliseconds_of(median),
        milliseconds_of(most)
    )
}
