use media::VideoCodec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativePipeline {
    pub codec: VideoCodec,
    pub bit_depth: u8,
    pub noise: bool,
}
