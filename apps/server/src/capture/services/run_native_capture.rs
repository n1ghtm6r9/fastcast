use crate::capture::{
    CaptureError, CaptureReport, CaptureTarget, NativeCapture, NativePipeline, TimerResolution,
};
use crate::hub::Hub;
use std::sync::atomic::AtomicBool;

pub fn run_native_capture(
    hub: &Hub,
    pipeline: NativePipeline,
    target: &CaptureTarget,
    bitrate_kbps: u32,
    stop: &AtomicBool,
) -> Result<CaptureReport, CaptureError> {
    let _timer = TimerResolution::one_millisecond();

    let mut capture = NativeCapture::open(hub, pipeline, target, bitrate_kbps, stop)
        .map_err(|cause| CaptureError::Native { frames: 0, cause })?;

    match capture.run() {
        Ok(()) => Ok(CaptureReport {
            frames: capture.frames,
        }),
        Err(cause) => Err(CaptureError::Native {
            frames: capture.frames,
            cause,
        }),
    }
}
