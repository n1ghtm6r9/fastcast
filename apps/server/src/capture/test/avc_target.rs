use crate::capture::CaptureTarget;
use media::VideoCodec;

pub fn avc_target() -> CaptureTarget {
    CaptureTarget {
        output: 0,
        fps: 60,
        codecs: vec![VideoCodec::Avc],
    }
}
