use crate::jni::test::{fake_env, jni_receive};
use std::time::{Duration, Instant};

#[test]
fn receive_on_an_unknown_link_returns_null_at_once() {
    let mut jvm = fake_env();
    let started = Instant::now();

    let received = jni_receive(&mut jvm, 999, 1000);

    assert_eq!(received, None);
    assert!(started.elapsed() < Duration::from_millis(50));
}
