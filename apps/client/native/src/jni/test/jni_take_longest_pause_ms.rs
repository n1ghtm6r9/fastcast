use crate::jni::Java_dev_fastcast_client_stream_NativeLink_takeLongestPauseMs;
use crate::jni::test::FakeEnv;
use jni_sys::{jint, jlong};
use std::ptr::null_mut;

pub(crate) fn jni_take_longest_pause_ms(jvm: &mut FakeEnv, link: jlong) -> jint {
    Java_dev_fastcast_client_stream_NativeLink_takeLongestPauseMs(jvm.pointer(), null_mut(), link)
}
