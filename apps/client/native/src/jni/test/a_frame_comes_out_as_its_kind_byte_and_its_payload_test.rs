use crate::jni::test::{connected_jni_link, jni_close, jni_receive};
use wire::FrameKind;

#[test]
fn a_frame_comes_out_as_its_kind_byte_and_its_payload() {
    let (mut server, mut jvm, link, peer) = connected_jni_link();
    let config = b"{\"type\":\"config\"}";

    server.send_frame(peer, FrameKind::Video, true, &[1, 2, 3], 0);

    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(vec![0, 1, 2, 3]));

    server.send_frame(peer, FrameKind::Config, false, config, 0);

    let mut expected = vec![1];
    expected.extend_from_slice(config);
    assert_eq!(jni_receive(&mut jvm, link, 1000), Some(expected));

    jni_close(&mut jvm, link);
}
