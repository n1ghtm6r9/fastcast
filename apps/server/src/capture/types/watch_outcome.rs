use crate::capture::{CaptureError, CaptureReport};

pub enum WatchOutcome {
    Finished(Result<CaptureReport, CaptureError>),
    Idle,
    Changed,
}
