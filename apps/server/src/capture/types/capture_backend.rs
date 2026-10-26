use crate::capture::{CaptureError, CaptureReport, CaptureTarget, Screen};
use crate::hub::{Hub, ViewerRequest};
use std::future::Future;
use std::sync::Arc;

pub trait CaptureBackend: Send + Sync + 'static {
    type Pipeline: Send + Sync;

    fn prepare(
        &self,
        request: &ViewerRequest,
    ) -> impl Future<Output = Result<Self::Pipeline, CaptureError>> + Send;

    fn open_screen(&self, request: &ViewerRequest) -> impl Future<Output = Screen> + Send;

    fn run(
        &self,
        hub: &Arc<Hub>,
        pipeline: &Self::Pipeline,
        target: &CaptureTarget,
    ) -> impl Future<Output = Result<CaptureReport, CaptureError>> + Send;
}
