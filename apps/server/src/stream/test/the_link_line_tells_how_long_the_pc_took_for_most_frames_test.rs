use crate::stream::sent_line;
use std::time::Duration;

#[test]
fn the_link_line_tells_how_long_the_pc_took_for_most_frames() {
    let mut delays: Vec<Duration> = [7, 3, 10, 1, 5, 9, 2, 8, 4, 6]
        .into_iter()
        .map(Duration::from_millis)
        .collect();

    assert_eq!(
        sent_line(&mut delays),
        "; the PC sent frames 5.0 ms after the screen showed them, 90% within 9.0 ms"
    );
}
