use crate::jni::link_of;
use crate::jni::test::{fake_env, jni_close, jni_open, jni_socket_drops};
use crate::link::test::FakeServer;

#[test]
fn an_opened_link_reports_the_drops_of_its_socket() {
    let server = FakeServer::new();
    let mut jvm = fake_env();
    let link = jni_open(&mut jvm, &server, 5);

    let reported = jni_socket_drops(&mut jvm, link);

    let actual = link_of(link).unwrap().socket_drops();
    assert_eq!(
        reported,
        actual.map_or(-1, |drops| i32::try_from(drops).unwrap())
    );

    jni_close(&mut jvm, link);
}
