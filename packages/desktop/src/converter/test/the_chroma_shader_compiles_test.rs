use crate::converter::{CHROMA_SHADER_SOURCE, PIXEL_SHADER_TARGET, compile_shader};

#[test]
fn the_chroma_shader_compiles() {
    let bytecode = compile_shader(CHROMA_SHADER_SOURCE, PIXEL_SHADER_TARGET).unwrap();

    assert!(!bytecode.is_empty());
}
