use crate::jni::MAX_RECEIVE_WAIT;
use crate::jni::test::{
    REPEATED_CALLS, connected_jni_link, fake_env, jni_close, jni_receive, jni_send_stats,
};
use crate::link::test::{TEST_SESSION, TEST_WAIT};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn closing_a_link_wakes_a_blocked_receive() {
    let (server, mut jvm, link, _peer) = connected_jni_link();
    let blocked = thread::spawn(move || {
        let mut jvm = fake_env();
        let started = Instant::now();
        let received = jni_receive(&mut jvm, link, i32::MAX);

        (received, started.elapsed())
    });
    thread::sleep(Duration::from_millis(100));

    jni_close(&mut jvm, link);

    let (received, blocked_for) = blocked.join().unwrap();
    assert_eq!(received, None);
    assert!(
        blocked_for < MAX_RECEIVE_WAIT / 2,
        "the receive waited {blocked_for:?}"
    );

    let started = Instant::now();
    for _ in 0..REPEATED_CALLS {
        assert_eq!(jni_receive(&mut jvm, link, 1000), None);
        assert!(!jni_send_stats(&mut jvm, link, b"{}"));
    }
    assert!(started.elapsed() < TEST_WAIT);

    assert_eq!(server.expect_bye(), TEST_SESSION);
}
