use crate::display::DisplayMode;
use media::VideoCodec;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ViewerRequest {
    pub mode: Option<DisplayMode>,
    pub codecs: Vec<VideoCodec>,
}
