use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_says_what_was_not_measured() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    let line = stats
        .line(start + Duration::from_secs(5))
        .expect("five seconds passed");

    assert!(
        line.contains("encoding took n/a; waits overran by n/a"),
        "{line}"
    );
}
