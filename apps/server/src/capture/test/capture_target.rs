use crate::capture::CaptureTarget;

pub fn capture_target() -> CaptureTarget {
    CaptureTarget { output: 0, fps: 60 }
}
