use jni_sys::{JNIEnv, jclass, jint};
use wire::PROTOCOL_VERSION;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_protocolVersion(
    _env: *mut JNIEnv,
    _class: jclass,
) -> jint {
    jint::from(PROTOCOL_VERSION)
}
