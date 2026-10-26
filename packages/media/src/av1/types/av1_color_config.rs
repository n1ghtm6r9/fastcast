#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Av1ColorConfig {
    pub high_bitdepth: bool,
    pub twelve_bit: bool,
    pub monochrome: bool,
    pub subsampling_x: bool,
    pub subsampling_y: bool,
    pub chroma_sample_position: u8,
}
