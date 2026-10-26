use crate::jni::test::{fake_env, jni_close, jni_open};
use crate::link::test::FakeServer;

#[test]
fn every_opened_link_gets_its_own_id() {
    let server = FakeServer::new();
    let mut jvm = fake_env();

    let first = jni_open(&mut jvm, &server, 1);
    let second = jni_open(&mut jvm, &server, 2);

    assert!(first > 0);
    assert!(second > 0);
    assert_ne!(first, second);

    jni_close(&mut jvm, first);
    jni_close(&mut jvm, second);
}
