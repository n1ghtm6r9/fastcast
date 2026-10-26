use crate::jni::test::{fake_env, jni_send_stats};

#[test]
fn stats_on_an_unknown_link_are_refused() {
    let mut jvm = fake_env();

    assert!(!jni_send_stats(&mut jvm, 999, b"{}"));
}
