use crate::link::test::HandDrivenLink;
use std::time::Duration;

#[test]
fn a_stalled_link_sends_one_hello_and_no_burst() {
    let mut link = HandDrivenLink::new();

    for milliseconds in [0, 10_000, 10_001, 10_100, 10_249, 10_250] {
        link.hello_at(Duration::from_millis(milliseconds));
    }

    assert_eq!(link.datagrams_sent(), 3);
    for _ in 0..3 {
        link.server.expect_hello();
    }
}
