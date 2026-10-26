use crate::jni::link_of;
use crate::jni::test::{fake_env, jni_close, jni_open, jni_receive_buffer_bytes};
use crate::link::test::FakeServer;

#[test]
fn an_opened_link_reports_its_receive_buffer() {
    let server = FakeServer::new();
    let mut jvm = fake_env();
    let link = jni_open(&mut jvm, &server, 5);
    assert!(link > 0);

    let reported = jni_receive_buffer_bytes(&mut jvm, link);

    let actual = link_of(link).unwrap().receive_buffer_bytes();
    assert!(reported > 0);
    assert_eq!(u32::try_from(reported).unwrap(), actual);

    jni_close(&mut jvm, link);
}
