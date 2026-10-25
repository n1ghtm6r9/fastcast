use crate::converter::{PIXEL_SHADER_TARGET, compile_shader};
use crate::gpu::DesktopError;

#[test]
fn a_shader_with_a_syntax_error_reports_the_compiler_message() {
    let error = compile_shader(
        "float4 main( : SV_Target { return 0; }",
        PIXEL_SHADER_TARGET,
    )
    .unwrap_err();

    match error {
        DesktopError::ShaderCompile(message) => assert!(message.contains("syntax error")),
        other => panic!("expected a compile error, got {other}"),
    }
}
