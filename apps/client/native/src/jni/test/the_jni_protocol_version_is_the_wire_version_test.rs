use crate::jni::Java_dev_fastcast_client_stream_NativeLink_protocolVersion;

#[test]
fn the_jni_protocol_version_is_the_wire_version() {
    let version = Java_dev_fastcast_client_stream_NativeLink_protocolVersion(
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    assert_eq!(version, i32::from(wire::PROTOCOL_VERSION));
}
