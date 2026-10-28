use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_tells_how_far_waits_overran() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    for late in 0..10_u64 {
        stats.waited(Duration::from_millis(late));
    }

    let line = stats
        .line(start + Duration::from_secs(5))
        .expect("five seconds passed");
    assert!(
        line.contains("waits overran by 4.0 ms, 90% within 8.0 ms, longest 9.0 ms"),
        "{line}"
    );
}
