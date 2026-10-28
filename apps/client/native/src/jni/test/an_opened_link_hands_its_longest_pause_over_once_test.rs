use crate::jni::link_of;
use crate::jni::test::{fake_env, jni_close, jni_open, jni_take_longest_pause_ms};
use crate::link::note_turn;
use crate::link::test::FakeServer;
use std::time::{Duration, Instant};

#[test]
fn an_opened_link_hands_its_longest_pause_over_once() {
    let server = FakeServer::new();
    let mut jvm = fake_env();
    let link = jni_open(&mut jvm, &server, 5);
    let start = Instant::now();
    let mut last_turn = start;
    note_turn(
        &link_of(link).unwrap().shared,
        &mut last_turn,
        start + Duration::from_secs(5),
    );

    assert_eq!(jni_take_longest_pause_ms(&mut jvm, link), 5_000);
    assert!(jni_take_longest_pause_ms(&mut jvm, link) < 5_000);

    jni_close(&mut jvm, link);
}
