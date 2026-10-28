use crate::jni::Java_dev_fastcast_client_stream_NativeLink_open;
use crate::jni::test::{FakeEnv, ints};
use jni_sys::{JNI_TRUE, jlong, jstring};
use std::ptr::null_mut;

pub(crate) fn jni_open_at(jvm: &mut FakeEnv, host: jstring, port: i32, nonce: i32) -> jlong {
    let codecs = ints(&[2, 8]);

    Java_dev_fastcast_client_stream_NativeLink_open(
        jvm.pointer(),
        null_mut(),
        host,
        port,
        nonce,
        1920,
        1080,
        60,
        codecs.pointer(),
        JNI_TRUE,
    )
}
