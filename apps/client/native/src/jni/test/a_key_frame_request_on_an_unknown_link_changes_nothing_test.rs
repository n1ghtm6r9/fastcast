use crate::jni::test::{connected_jni_link, jni_close, jni_request_key_frame};
use crate::link::test::ReceivedPacket;
use std::time::Duration;

#[test]
fn a_key_frame_request_on_an_unknown_link_changes_nothing() {
    let (server, mut jvm, link, _peer) = connected_jni_link();

    jni_request_key_frame(&mut jvm, 999);

    let requested = server
        .packets_within(Duration::from_millis(150))
        .into_iter()
        .any(|packet| {
            matches!(
                packet,
                ReceivedPacket::Feedback { feedback, .. } if feedback.keyframe_needed
            )
        });
    assert!(!requested, "an unknown link made another link ask");

    jni_close(&mut jvm, link);
}
