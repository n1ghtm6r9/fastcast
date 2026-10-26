use crate::jni::{guarded, link_of};
use jni_sys::{JNIEnv, jclass, jlong};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_requestKeyFrame(
    _env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
) {
    guarded((), || {
        if let Some(link) = link_of(link) {
            link.request_key_frame();
        }
    })
}
