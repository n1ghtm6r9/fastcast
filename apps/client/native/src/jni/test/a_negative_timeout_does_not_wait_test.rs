use crate::jni::test::{connected_jni_link, jni_close, jni_receive};
use std::time::{Duration, Instant};

#[test]
fn a_negative_timeout_does_not_wait() {
    let (_server, mut jvm, link, _peer) = connected_jni_link();
    let started = Instant::now();

    assert_eq!(jni_receive(&mut jvm, link, -5), None);

    assert!(started.elapsed() < Duration::from_millis(50));

    jni_close(&mut jvm, link);
}
