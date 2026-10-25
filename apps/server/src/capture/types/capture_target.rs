use media::VideoCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureTarget {
    pub output: u32,
    pub fps: u32,
    pub codecs: Vec<VideoCodec>,
}
