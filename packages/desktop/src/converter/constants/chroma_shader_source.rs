pub const CHROMA_SHADER_SOURCE: &str = concat!(
    include_str!("../shaders/composite.hlsli"),
    include_str!("../shaders/chroma_ps.hlsl")
);
