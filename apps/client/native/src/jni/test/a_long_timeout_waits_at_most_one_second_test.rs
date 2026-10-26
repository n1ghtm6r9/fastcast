use crate::jni::test::{connected_jni_link, fake_env, jni_close, jni_receive};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn a_long_timeout_waits_at_most_one_second() {
    let (_server, mut jvm, link, _peer) = connected_jni_link();
    let (sender, receiver) = mpsc::channel();
    let started = Instant::now();

    thread::spawn(move || {
        let mut jvm = fake_env();
        let received = jni_receive(&mut jvm, link, i32::MAX);

        sender.send(received).unwrap();
    });

    let received = receiver
        .recv_timeout(Duration::from_secs(5))
        .expect("receive waited longer than one second");
    let waited = started.elapsed();

    assert_eq!(received, None);
    assert!(
        waited >= Duration::from_millis(900),
        "waited only {waited:?}"
    );

    jni_close(&mut jvm, link);
}
