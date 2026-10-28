use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_tells_the_longest_pause_between_frames() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    for at in [0, 10, 40, 50] {
        stats.encoded(
            start + Duration::from_millis(at),
            Duration::from_millis(1),
            false,
        );
    }

    let line = stats
        .line(start + Duration::from_secs(5))
        .expect("five seconds passed");
    assert!(
        line.ends_with("the longest pause between frames was 30.0 ms"),
        "{line}"
    );
}
