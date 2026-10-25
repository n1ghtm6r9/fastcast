use crate::capture::{CaptureError, CaptureReport, CaptureTarget, Screen};
use crate::hub::{Hub, ViewerRequest};
use std::future::Future;

pub trait CaptureBackend: Send + Sync + 'static {
    fn open_screen(&self, request: &ViewerRequest) -> impl Future<Output = Screen> + Send;

    fn run(
        &self,
        hub: &Hub,
        target: &CaptureTarget,
    ) -> impl Future<Output = Result<CaptureReport, CaptureError>> + Send;
}
