use crate::jni::Java_dev_fastcast_client_stream_NativeLink_inboxDrops;
use crate::jni::test::FakeEnv;
use jni_sys::{jint, jlong};
use std::ptr::null_mut;

pub(crate) fn jni_inbox_drops(jvm: &mut FakeEnv, link: jlong) -> jint {
    Java_dev_fastcast_client_stream_NativeLink_inboxDrops(jvm.pointer(), null_mut(), link)
}
