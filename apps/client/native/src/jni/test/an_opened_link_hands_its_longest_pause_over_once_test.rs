use crate::jni::link_of;
use crate::jni::test::{fake_env, jni_close, jni_open, jni_take_longest_pause_ms};
use crate::link::test::FakeServer;
use crate::link::{LinkState, note_turn};
use std::time::{Duration, Instant};

#[test]
fn an_opened_link_hands_its_longest_pause_over_once() {
    let server = FakeServer::new();
    let mut jvm = fake_env();
    let link = jni_open(&mut jvm, &server, 5);
    let start = Instant::now();
    note_turn(
        &link_of(link).unwrap().shared,
        &mut LinkState::new(start),
        start + Duration::from_secs(5),
    );

    assert_eq!(jni_take_longest_pause_ms(&mut jvm, link), 5_000);
    assert!(jni_take_longest_pause_ms(&mut jvm, link) < 5_000);

    jni_close(&mut jvm, link);
}
