use crate::capture::{CaptureError, CaptureReport};
use crate::hub::Hub;
use std::future::Future;

pub trait CaptureBackend: Send + Sync + 'static {
    fn run(&self, hub: &Hub) -> impl Future<Output = Result<CaptureReport, CaptureError>> + Send;
}
