use crate::jni::test::{connected_jni_link, jni_close, jni_request_key_frame};

#[test]
fn a_key_frame_request_through_jni_reaches_the_server() {
    let (server, mut jvm, link, _peer) = connected_jni_link();

    jni_request_key_frame(&mut jvm, link);

    server.expect_feedback_where(|feedback| feedback.keyframe_needed);

    jni_close(&mut jvm, link);
}
