use crate::display::DisplayMode;
use media::VideoCodec;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ViewerRequest {
    pub mode: Option<DisplayMode>,
    pub codecs: Vec<VideoCodec>,
    pub ten_bit_codecs: Vec<VideoCodec>,
}
