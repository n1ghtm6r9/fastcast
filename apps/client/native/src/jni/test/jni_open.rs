use crate::jni::test::{FakeEnv, jni_open_at, text};
use crate::link::test::FakeServer;
use jni_sys::jlong;

pub(crate) fn jni_open(jvm: &mut FakeEnv, server: &FakeServer, nonce: i32) -> jlong {
    let host = text("127.0.0.1");

    jni_open_at(
        jvm,
        host.pointer(),
        i32::from(server.address().port()),
        nonce,
    )
}
