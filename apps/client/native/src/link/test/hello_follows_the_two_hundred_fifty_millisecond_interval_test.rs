use crate::link::test::HandDrivenLink;
use std::time::Duration;

#[test]
fn hello_follows_the_two_hundred_fifty_millisecond_interval() {
    let mut link = HandDrivenLink::new();

    for milliseconds in [0, 249, 250, 499, 500] {
        link.hello_at(Duration::from_millis(milliseconds));
    }

    assert_eq!(link.datagrams_sent(), 3);
    for _ in 0..3 {
        link.server.expect_hello();
    }
}
