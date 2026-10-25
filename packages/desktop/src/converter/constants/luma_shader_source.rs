pub const LUMA_SHADER_SOURCE: &str = concat!(
    include_str!("../shaders/composite.hlsli"),
    include_str!("../shaders/luma_ps.hlsl")
);
