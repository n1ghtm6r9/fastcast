use crate::jni::Java_dev_fastcast_client_stream_NativeLink_socketDrops;
use crate::jni::test::FakeEnv;
use jni_sys::{jint, jlong};
use std::ptr::null_mut;

pub(crate) fn jni_socket_drops(jvm: &mut FakeEnv, link: jlong) -> jint {
    Java_dev_fastcast_client_stream_NativeLink_socketDrops(jvm.pointer(), null_mut(), link)
}
