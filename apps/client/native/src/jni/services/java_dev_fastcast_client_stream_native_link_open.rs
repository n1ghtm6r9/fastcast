use crate::jni::{guarded, ints_of, register_link, string_of};
use crate::link::{Link, LinkTarget, hello_of};
use jni_sys::{JNIEnv, jclass, jint, jintArray, jlong, jstring};
use std::net::{IpAddr, SocketAddr};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_open(
    env: *mut JNIEnv,
    _class: jclass,
    host: jstring,
    port: jint,
    nonce: jint,
    width: jint,
    height: jint,
    fps: jint,
    codecs: jintArray,
) -> jlong {
    guarded(0, || {
        let Some(host) = string_of(env, host) else {
            return 0;
        };
        let Ok(address) = host.parse::<IpAddr>() else {
            return 0;
        };
        let port = match u16::try_from(port) {
            Ok(port) if port > 0 => port,
            _ => return 0,
        };
        let hello = hello_of(nonce as u32, width, height, fps, &ints_of(env, codecs));
        let target = LinkTarget {
            server: SocketAddr::new(address, port),
            hello,
        };

        match Link::open(target) {
            Ok(link) => register_link(link),
            Err(_) => 0,
        }
    })
}
