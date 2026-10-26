use crate::jni::test::{fake_env, jni_open_at, text};
use std::ptr::null_mut;

#[test]
fn a_host_that_is_not_an_ip_address_opens_nothing() {
    let mut jvm = fake_env();
    let name = text("tv.local");
    let address = text("127.0.0.1");

    assert_eq!(jni_open_at(&mut jvm, name.pointer(), 3278, 5), 0);
    assert_eq!(jni_open_at(&mut jvm, null_mut(), 3278, 5), 0);
    assert_eq!(jni_open_at(&mut jvm, address.pointer(), 0, 5), 0);
    assert_eq!(jni_open_at(&mut jvm, address.pointer(), -1, 5), 0);
    assert_eq!(jni_open_at(&mut jvm, address.pointer(), 65_536, 5), 0);
    assert_eq!(jni_open_at(&mut jvm, address.pointer(), 70_000, 5), 0);
}
