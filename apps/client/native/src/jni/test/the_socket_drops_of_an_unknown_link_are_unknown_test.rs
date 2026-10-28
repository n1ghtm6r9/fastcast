use crate::jni::test::{fake_env, jni_socket_drops};

#[test]
fn the_socket_drops_of_an_unknown_link_are_unknown() {
    let mut jvm = fake_env();

    assert_eq!(jni_socket_drops(&mut jvm, 999), -1);
}
