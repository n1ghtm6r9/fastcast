use crate::jni::test::{REPEATED_CALLS, connected_jni_link, jni_close, jni_receive};
use crate::link::test::TEST_WAIT;
use std::time::Instant;

#[test]
fn a_negative_timeout_does_not_wait() {
    let (_server, mut jvm, link, _peer) = connected_jni_link();
    let started = Instant::now();

    for _ in 0..REPEATED_CALLS {
        assert_eq!(jni_receive(&mut jvm, link, -5), None);
    }

    assert!(started.elapsed() < TEST_WAIT);

    jni_close(&mut jvm, link);
}
