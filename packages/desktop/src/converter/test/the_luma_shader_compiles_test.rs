use crate::converter::{LUMA_SHADER_SOURCE, PIXEL_SHADER_TARGET, compile_shader};

#[test]
fn the_luma_shader_compiles() {
    let bytecode = compile_shader(LUMA_SHADER_SOURCE, PIXEL_SHADER_TARGET).unwrap();

    assert!(!bytecode.is_empty());
}
