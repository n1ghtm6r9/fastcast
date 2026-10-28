use crate::duplication::ticks_as_duration;
use std::time::Duration;

#[test]
fn ticks_of_one_second_last_a_second() {
    assert_eq!(
        ticks_as_duration(10_000_000, 10_000_000),
        Some(Duration::from_secs(1))
    );
}
