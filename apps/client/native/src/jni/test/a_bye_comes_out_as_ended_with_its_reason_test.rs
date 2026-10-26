use crate::jni::test::{REPEATED_CALLS, connected_jni_link, jni_close, jni_receive};
use crate::link::test::TEST_WAIT;
use std::time::Instant;

#[test]
fn a_bye_comes_out_as_ended_with_its_reason() {
    let (mut server, mut jvm, link, peer) = connected_jni_link();

    server.send_bye(peer);

    let mut expected = vec![3];
    expected.extend_from_slice(b"Closed by server");
    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(expected));

    let started = Instant::now();
    for _ in 0..REPEATED_CALLS {
        assert_eq!(jni_receive(&mut jvm, link, 1000), None);
    }
    assert!(started.elapsed() < TEST_WAIT);

    jni_close(&mut jvm, link);
}
