use crate::jni::test::{fake_env, jni_inbox_drops};

#[test]
fn the_inbox_drops_of_an_unknown_link_are_unknown() {
    let mut jvm = fake_env();

    assert_eq!(jni_inbox_drops(&mut jvm, 999), -1);
}
