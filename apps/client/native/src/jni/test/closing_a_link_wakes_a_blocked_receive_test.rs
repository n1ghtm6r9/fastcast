use crate::jni::test::{connected_jni_link, fake_env, jni_close, jni_receive, jni_send_stats};
use crate::link::test::TEST_SESSION;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn closing_a_link_wakes_a_blocked_receive() {
    let (server, mut jvm, link, _peer) = connected_jni_link();
    let blocked = thread::spawn(move || {
        let mut jvm = fake_env();
        let received = jni_receive(&mut jvm, link, 1000);

        (received, Instant::now())
    });
    thread::sleep(Duration::from_millis(100));

    let closing = Instant::now();
    jni_close(&mut jvm, link);

    let (received, returned_at) = blocked.join().unwrap();
    assert_eq!(received, None);
    assert!(returned_at.saturating_duration_since(closing) < Duration::from_millis(500));

    let started = Instant::now();
    assert_eq!(jni_receive(&mut jvm, link, 1000), None);
    assert!(!jni_send_stats(&mut jvm, link, b"{}"));
    assert!(started.elapsed() < Duration::from_millis(50));

    assert_eq!(server.expect_bye(), TEST_SESSION);
}
