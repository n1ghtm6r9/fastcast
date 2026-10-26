use crate::jni::Java_dev_fastcast_client_stream_NativeLink_close;
use crate::jni::test::FakeEnv;
use jni_sys::jlong;
use std::ptr::null_mut;

pub(crate) fn jni_close(jvm: &mut FakeEnv, link: jlong) {
    Java_dev_fastcast_client_stream_NativeLink_close(jvm.pointer(), null_mut(), link);
}
