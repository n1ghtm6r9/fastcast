use crate::jni::test::{connected_jni_link, jni_close, jni_receive};

#[test]
fn a_notice_comes_out_as_its_kind_byte_and_its_text() {
    let (mut server, mut jvm, link, peer) = connected_jni_link();

    server.send_notice(peer, "{}");

    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(vec![4, b'{', b'}']));

    jni_close(&mut jvm, link);
}
