use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_starts_over_after_each_report() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);
    stats.encoded(start, Duration::from_millis(9), true);
    stats.encoded(
        start + Duration::from_secs(3),
        Duration::from_millis(9),
        true,
    );
    stats.encoded(
        start + Duration::from_millis(4_900),
        Duration::from_millis(9),
        false,
    );
    stats.waited(Duration::from_millis(7));
    stats.replaced();
    let _first = stats.line(start + Duration::from_secs(5));

    stats.encoded(
        start + Duration::from_secs(5),
        Duration::from_millis(1),
        false,
    );

    let line = stats
        .line(start + Duration::from_secs(10))
        .expect("five more seconds passed");
    assert_eq!(
        line,
        "capture: 0.2 frames/s, 0.0 of them new pictures, 0 pictures replaced before encoding; \
         encoding took 1.0 ms, 90% within 1.0 ms, longest 1.0 ms; waits overran by n/a; \
         the longest pause between frames was 100.0 ms"
    );
}
