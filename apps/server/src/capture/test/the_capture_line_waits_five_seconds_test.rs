use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_waits_five_seconds() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    stats.encoded(start, Duration::from_millis(2), true);

    assert_eq!(stats.line(start + Duration::from_millis(4_999)), None);
}
