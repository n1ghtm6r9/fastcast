use crate::jni::{guarded, link_of};
use jni_sys::{JNIEnv, jclass, jint, jlong};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_receiveBufferBytes(
    _env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
) -> jint {
    guarded(0, || {
        link_of(link).map_or(0, |link| {
            jint::try_from(link.receive_buffer_bytes()).unwrap_or(jint::MAX)
        })
    })
}
