use crate::jni::test::{fake_env, jni_send_notice};

#[test]
fn notices_on_an_unknown_link_are_refused() {
    let mut jvm = fake_env();

    assert!(!jni_send_notice(&mut jvm, 999, b"{}"));
}
