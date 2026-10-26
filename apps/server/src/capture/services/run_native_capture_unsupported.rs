use crate::capture::{CaptureError, CaptureReport, CaptureTarget, NativeError, NativePipeline};
use crate::hub::Hub;
use std::sync::atomic::AtomicBool;

pub fn run_native_capture(
    _hub: &Hub,
    _pipeline: NativePipeline,
    _target: &CaptureTarget,
    _bitrate_kbps: u32,
    _stop: &AtomicBool,
) -> Result<CaptureReport, CaptureError> {
    Err(CaptureError::Native {
        frames: 0,
        cause: NativeError::Unsupported,
    })
}
