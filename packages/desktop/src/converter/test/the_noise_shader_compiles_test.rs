use crate::converter::{NOISE_SHADER_SOURCE, PIXEL_SHADER_TARGET, compile_shader};

#[test]
fn the_noise_shader_compiles() {
    let bytecode = compile_shader(NOISE_SHADER_SOURCE, PIXEL_SHADER_TARGET).unwrap();

    assert!(!bytecode.is_empty());
}
