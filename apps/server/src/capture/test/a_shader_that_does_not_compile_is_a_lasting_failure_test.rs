use crate::capture::NativeError;
use desktop::DesktopError;

#[test]
fn a_shader_that_does_not_compile_is_a_lasting_failure() {
    let error = NativeError::Desktop(DesktopError::ShaderCompile("a test message".to_owned()));

    assert!(error.is_lasting());
}
