use crate::capture::NativeError;

#[test]
fn a_stream_the_server_cannot_describe_is_a_lasting_failure() {
    assert!(NativeError::UndescribableStream.is_lasting());
}
