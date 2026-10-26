use crate::capture::NativeError;

#[test]
fn a_key_frame_without_parameter_sets_is_described() {
    assert_eq!(
        NativeError::UndescribableStream.to_string(),
        "the encoder wrote a key frame without parameter sets the server can describe"
    );
}
