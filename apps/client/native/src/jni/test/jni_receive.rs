use crate::jni::Java_dev_fastcast_client_stream_NativeLink_receive;
use crate::jni::test::{FakeEnv, bytes_back};
use jni_sys::jlong;
use std::ptr::null_mut;

pub(crate) fn jni_receive(jvm: &mut FakeEnv, link: jlong, timeout_ms: i32) -> Option<Vec<u8>> {
    let array = Java_dev_fastcast_client_stream_NativeLink_receive(
        jvm.pointer(),
        null_mut(),
        link,
        timeout_ms,
    );

    if array.is_null() {
        None
    } else {
        Some(bytes_back(array))
    }
}
