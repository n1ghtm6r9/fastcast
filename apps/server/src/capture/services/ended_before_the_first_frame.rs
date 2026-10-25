use crate::capture::{CaptureError, CaptureReport};

pub fn ended_before_the_first_frame(result: &Result<CaptureReport, CaptureError>) -> bool {
    matches!(result, Ok(report) if report.frames == 0)
}
