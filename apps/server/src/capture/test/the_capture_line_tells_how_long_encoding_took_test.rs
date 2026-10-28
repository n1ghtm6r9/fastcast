use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_tells_how_long_encoding_took() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    for took in 1..=10_u64 {
        stats.encoded(start, Duration::from_millis(took), false);
    }

    let line = stats
        .line(start + Duration::from_secs(5))
        .expect("five seconds passed");
    assert!(
        line.contains("encoding took 5.0 ms, 90% within 9.0 ms, longest 10.0 ms"),
        "{line}"
    );
}
