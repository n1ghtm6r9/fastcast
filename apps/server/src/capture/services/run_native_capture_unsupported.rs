use crate::capture::{CaptureError, CaptureReport, CaptureTarget, NativeError};
use crate::hub::Hub;
use media::VideoCodec;
use std::sync::atomic::AtomicBool;

pub fn run_native_capture(
    _hub: &Hub,
    _codec: VideoCodec,
    _target: &CaptureTarget,
    _bitrate_kbps: u32,
    _stop: &AtomicBool,
) -> Result<CaptureReport, CaptureError> {
    Err(CaptureError::Native {
        frames: 0,
        cause: NativeError::Unsupported,
    })
}
