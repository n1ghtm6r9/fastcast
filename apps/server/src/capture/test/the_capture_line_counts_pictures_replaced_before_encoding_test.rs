use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

#[test]
fn the_capture_line_counts_pictures_replaced_before_encoding() {
    let start = Instant::now();
    let mut stats = CaptureStats::new(start);

    stats.replaced();
    stats.replaced();

    let line = stats
        .line(start + Duration::from_secs(5))
        .expect("five seconds passed");
    assert!(
        line.contains("2 pictures replaced before encoding"),
        "{line}"
    );
}
