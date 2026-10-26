use crate::jni::test::{fake_env, jni_close, jni_open, jni_receive};
use crate::link::test::{FakeServer, TEST_SESSION};
use wire::{Codec, CodecSupport, Hello};

#[test]
fn an_opened_link_reports_connected_through_receive() {
    let mut server = FakeServer::new();
    let mut jvm = fake_env();

    let link = jni_open(&mut jvm, &server, 5);
    assert!(link > 0);

    let (hello, peer) = server.expect_hello();
    assert_eq!(
        hello,
        Hello {
            nonce: 5,
            width: 1920,
            height: 1080,
            fps: 60,
            codecs: vec![CodecSupport {
                codec: Codec::Hevc,
                max_bit_depth: 8,
            }],
        }
    );

    server.welcome(peer, 5, TEST_SESSION);

    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(vec![2]));

    jni_close(&mut jvm, link);
}
