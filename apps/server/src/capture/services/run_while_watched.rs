use crate::capture::{
    CaptureBackend, CaptureTarget, IDLE_GRACE, WatchOutcome, request_changed, viewers_gone_for,
};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use tokio::sync::watch;

pub async fn run_while_watched<B: CaptureBackend>(
    hub: &Arc<Hub>,
    backend: &B,
    pipeline: &B::Pipeline,
    target: &CaptureTarget,
    viewers: &mut watch::Receiver<usize>,
    requests: &mut watch::Receiver<ViewerRequest>,
) -> WatchOutcome {
    tokio::select! {
        result = backend.run(hub, pipeline, target) => WatchOutcome::Finished(result),
        () = viewers_gone_for(viewers, IDLE_GRACE) => WatchOutcome::Idle,
        () = request_changed(requests) => WatchOutcome::Changed,
    }
}
