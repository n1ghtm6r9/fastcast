use crate::jni::{UNKNOWN_COUNT, guarded, link_of};
use jni_sys::{JNIEnv, jclass, jint, jlong};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_inboxDrops(
    _env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
) -> jint {
    guarded(UNKNOWN_COUNT, || {
        link_of(link).map_or(UNKNOWN_COUNT, |link| {
            jint::try_from(link.inbox_drops()).unwrap_or(jint::MAX)
        })
    })
}
