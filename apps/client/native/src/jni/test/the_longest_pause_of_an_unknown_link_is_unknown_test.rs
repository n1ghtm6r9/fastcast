use crate::jni::test::{fake_env, jni_take_longest_pause_ms};

#[test]
fn the_longest_pause_of_an_unknown_link_is_unknown() {
    let mut jvm = fake_env();

    assert_eq!(jni_take_longest_pause_ms(&mut jvm, 999), -1);
}
