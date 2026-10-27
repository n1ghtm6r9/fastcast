use crate::jni::Java_dev_fastcast_client_stream_NativeLink_sendNotice;
use crate::jni::test::{FakeEnv, bytes};
use jni_sys::jlong;
use std::ptr::null_mut;

pub(crate) fn jni_send_notice(jvm: &mut FakeEnv, link: jlong, text: &[u8]) -> bool {
    let array = bytes(text);

    Java_dev_fastcast_client_stream_NativeLink_sendNotice(
        jvm.pointer(),
        null_mut(),
        link,
        array.pointer(),
    )
}
