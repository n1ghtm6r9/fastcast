use crate::converter::{VERTEX_SHADER_SOURCE, VERTEX_SHADER_TARGET, compile_shader};

#[test]
fn the_vertex_shader_compiles() {
    let bytecode = compile_shader(VERTEX_SHADER_SOURCE, VERTEX_SHADER_TARGET).unwrap();

    assert!(!bytecode.is_empty());
}
