use crate::jni::Java_dev_fastcast_client_stream_NativeLink_receiveBufferBytes;
use crate::jni::test::FakeEnv;
use jni_sys::{jint, jlong};
use std::ptr::null_mut;

pub(crate) fn jni_receive_buffer_bytes(jvm: &mut FakeEnv, link: jlong) -> jint {
    Java_dev_fastcast_client_stream_NativeLink_receiveBufferBytes(jvm.pointer(), null_mut(), link)
}
