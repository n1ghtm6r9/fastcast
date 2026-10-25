use crate::web::ViewerTimings;

#[test]
fn the_silence_limit_spans_at_least_two_ping_intervals() {
    let timings = ViewerTimings::default();

    assert!(timings.silence_timeout >= timings.ping_interval * 2);
}
