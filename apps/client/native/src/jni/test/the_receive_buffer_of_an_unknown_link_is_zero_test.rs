use crate::jni::test::{fake_env, jni_receive_buffer_bytes};

#[test]
fn the_receive_buffer_of_an_unknown_link_is_zero() {
    let mut jvm = fake_env();

    assert_eq!(jni_receive_buffer_bytes(&mut jvm, 999), 0);
}
