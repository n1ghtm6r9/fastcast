use crate::jni::Java_dev_fastcast_client_stream_NativeLink_sendStats;
use crate::jni::test::{FakeEnv, bytes};
use jni_sys::jlong;
use std::ptr::null_mut;

pub(crate) fn jni_send_stats(jvm: &mut FakeEnv, link: jlong, text: &[u8]) -> bool {
    let array = bytes(text);

    Java_dev_fastcast_client_stream_NativeLink_sendStats(
        jvm.pointer(),
        null_mut(),
        link,
        array.pointer(),
    )
}
