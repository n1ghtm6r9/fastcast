use crate::jni::test::{REPEATED_CALLS, fake_env, jni_receive};
use crate::link::test::TEST_WAIT;
use std::time::Instant;

#[test]
fn receive_on_an_unknown_link_returns_null_at_once() {
    let mut jvm = fake_env();
    let started = Instant::now();

    for _ in 0..REPEATED_CALLS {
        assert_eq!(jni_receive(&mut jvm, 999, 1000), None);
    }

    assert!(started.elapsed() < TEST_WAIT);
}
