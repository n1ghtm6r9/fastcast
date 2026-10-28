use crate::link::test::opened_link;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn the_link_thread_notes_how_long_each_turn_takes() {
    let (server, link) = opened_link();
    server.expect_hello();

    sleep(Duration::from_millis(50));

    assert!(link.take_longest_pause_ms() >= 1);
}
