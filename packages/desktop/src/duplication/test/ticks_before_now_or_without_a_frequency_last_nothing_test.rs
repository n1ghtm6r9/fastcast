use crate::duplication::ticks_as_duration;

#[test]
fn ticks_before_now_or_without_a_frequency_last_nothing() {
    assert_eq!(ticks_as_duration(-1, 10_000_000), None);
    assert_eq!(ticks_as_duration(15, 0), None);
}
