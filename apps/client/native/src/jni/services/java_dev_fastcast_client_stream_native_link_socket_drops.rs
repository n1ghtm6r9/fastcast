use crate::jni::{UNKNOWN_COUNT, guarded, link_of};
use jni_sys::{JNIEnv, jclass, jint, jlong};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_socketDrops(
    _env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
) -> jint {
    guarded(UNKNOWN_COUNT, || {
        link_of(link)
            .and_then(|link| link.socket_drops())
            .map_or(UNKNOWN_COUNT, |drops| {
                jint::try_from(drops).unwrap_or(jint::MAX)
            })
    })
}
