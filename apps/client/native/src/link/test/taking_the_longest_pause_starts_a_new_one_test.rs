use crate::link::test::HandDrivenLink;
use std::time::Duration;

#[test]
fn taking_the_longest_pause_starts_a_new_one() {
    let mut link = HandDrivenLink::new();
    link.turn_at(Duration::from_millis(40));
    assert_eq!(link.shared.take_longest_pause_ms(), 40);

    link.turn_at(Duration::from_millis(42));

    assert_eq!(link.shared.take_longest_pause_ms(), 2);
}
