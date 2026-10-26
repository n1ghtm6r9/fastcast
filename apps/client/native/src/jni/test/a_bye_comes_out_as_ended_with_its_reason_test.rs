use crate::jni::test::{connected_jni_link, jni_close, jni_receive};
use std::time::{Duration, Instant};

#[test]
fn a_bye_comes_out_as_ended_with_its_reason() {
    let (mut server, mut jvm, link, peer) = connected_jni_link();

    server.send_bye(peer);

    let mut expected = vec![3];
    expected.extend_from_slice(b"Closed by server");
    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(expected));

    let started = Instant::now();
    assert_eq!(jni_receive(&mut jvm, link, 1000), None);
    assert!(started.elapsed() < Duration::from_millis(500));

    jni_close(&mut jvm, link);
}
