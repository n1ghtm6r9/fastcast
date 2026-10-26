use crate::jni::test::{connected_jni_link, jni_close, jni_send_stats};

#[test]
fn stats_are_sent_through_the_link() {
    let (server, mut jvm, link, _peer) = connected_jni_link();

    assert!(jni_send_stats(&mut jvm, link, b"{}"));

    assert_eq!(server.expect_stats(), "{}");

    jni_close(&mut jvm, link);
}
