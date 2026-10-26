use crate::jni::{bytes_of, guarded, link_of};
use jni_sys::{JNI_FALSE, JNIEnv, jboolean, jbyteArray, jclass, jlong};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_sendStats(
    env: *mut JNIEnv,
    _class: jclass,
    link: jlong,
    text: jbyteArray,
) -> jboolean {
    guarded(JNI_FALSE, || {
        let Some(link) = link_of(link) else {
            return JNI_FALSE;
        };
        let Some(text) = bytes_of(env, text) else {
            return JNI_FALSE;
        };

        link.send_stats(&text)
    })
}
