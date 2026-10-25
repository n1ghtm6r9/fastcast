use crate::capture::CaptureError;

pub enum PrepareOutcome<P> {
    Ready(P),
    Failed(CaptureError),
    Idle,
    Changed,
}
