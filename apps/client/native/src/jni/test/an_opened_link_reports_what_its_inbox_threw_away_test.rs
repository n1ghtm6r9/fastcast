use crate::jni::link_of;
use crate::jni::test::{fake_env, jni_close, jni_inbox_drops, jni_open};
use crate::link::test::FakeServer;
use std::sync::atomic::Ordering;

#[test]
fn an_opened_link_reports_what_its_inbox_threw_away() {
    let server = FakeServer::new();
    let mut jvm = fake_env();
    let link = jni_open(&mut jvm, &server, 5);
    link_of(link)
        .unwrap()
        .shared
        .inbox_drops
        .store(7, Ordering::Relaxed);

    assert_eq!(jni_inbox_drops(&mut jvm, link), 7);

    jni_close(&mut jvm, link);
}
