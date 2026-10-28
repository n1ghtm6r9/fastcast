use crate::link::test::HandDrivenLink;
use std::time::Duration;

#[test]
fn the_longest_pause_between_turns_of_the_link_is_kept() {
    let mut link = HandDrivenLink::new();

    link.turn_at(Duration::from_millis(5));
    link.turn_at(Duration::from_millis(35));
    link.turn_at(Duration::from_millis(45));

    assert_eq!(link.shared.take_longest_pause_ms(), 30);
}
