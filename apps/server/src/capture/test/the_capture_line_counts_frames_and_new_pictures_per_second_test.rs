use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_counts_frames_and_new_pictures_per_second() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    for index in 0..10_u64 {
        stats.encoded(
            start + Duration::from_millis(index * 100),
            Duration::from_millis(1),
            index < 2,
        );
    }

    let line = stats
        .line(start + Duration::from_secs(5))
        .expect("five seconds passed");
    assert!(
        line.starts_with("capture: 2.0 frames/s, 0.4 of them new pictures"),
        "{line}"
    );
}
