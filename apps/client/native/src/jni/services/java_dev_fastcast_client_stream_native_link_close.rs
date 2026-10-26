use crate::jni::{forget_link, guarded};
use jni_sys::{JNIEnv, jclass, jlong};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_close(
    _env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
) {
    guarded((), || {
        if let Some(link) = forget_link(link) {
            link.close();
        }
    })
}
