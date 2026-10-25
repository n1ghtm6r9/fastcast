use crate::capture::{CaptureError, CaptureReport};

pub fn encoder_failed(result: &Result<CaptureReport, CaptureError>) -> bool {
    matches!(result, Ok(report) if report.frames == 0)
}
