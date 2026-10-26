use crate::link::test::HandDrivenLink;
use std::time::Duration;

#[test]
fn feedback_follows_the_twenty_millisecond_interval() {
    let mut link = HandDrivenLink::new();
    link.welcome();

    for milliseconds in [0, 19, 20, 39, 40] {
        link.feedback_at(Duration::from_millis(milliseconds));
    }

    assert_eq!(link.datagrams_sent(), 3);
    for _ in 0..3 {
        link.server.expect_feedback();
    }
}
