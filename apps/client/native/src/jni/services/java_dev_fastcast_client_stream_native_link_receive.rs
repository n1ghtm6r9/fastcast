use crate::jni::{MAX_RECEIVE_WAIT, event_array, guarded, link_of};
use jni_sys::{JNIEnv, jbyteArray, jclass, jint, jlong};
use std::ptr::null_mut;
use std::time::Duration;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_receive(
    env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
    timeout_ms: jint,
) -> jbyteArray {
    guarded(null_mut(), || {
        let Some(link) = link_of(link) else {
            return null_mut();
        };
        let timeout = Duration::from_millis(timeout_ms.max(0) as u64).min(MAX_RECEIVE_WAIT);

        match link.next_event(timeout) {
            Some(event) => event_array(env, &event),
            None => null_mut(),
        }
    })
}
