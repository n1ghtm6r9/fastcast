use crate::duplication::ticks_as_duration;
use std::time::Duration;

#[test]
fn a_few_ticks_last_their_share_of_a_second() {
    assert_eq!(
        ticks_as_duration(15, 10_000_000),
        Some(Duration::from_nanos(1_500))
    );
}
