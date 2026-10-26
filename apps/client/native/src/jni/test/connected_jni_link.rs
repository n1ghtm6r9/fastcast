use crate::jni::test::{FakeEnv, fake_env, jni_open, jni_receive};
use crate::link::test::{FakeServer, TEST_NONCE, TEST_SESSION};
use jni_sys::jlong;
use std::net::SocketAddr;

pub(crate) fn connected_jni_link() -> (FakeServer, FakeEnv, jlong, SocketAddr) {
    let mut server = FakeServer::new();
    let mut jvm = fake_env();
    let link = jni_open(&mut jvm, &server, TEST_NONCE as i32);
    assert!(link > 0);

    let (_, peer) = server.expect_hello();
    server.welcome(peer, TEST_NONCE, TEST_SESSION);

    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(vec![2]));

    (server, jvm, link, peer)
}
