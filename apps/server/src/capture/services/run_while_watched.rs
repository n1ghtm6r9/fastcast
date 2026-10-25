use crate::capture::{
    CaptureBackend, CaptureTarget, IDLE_GRACE, WatchOutcome, request_changed, viewers_gone_for,
};
use crate::hub::{Hub, ViewerRequest};
use tokio::sync::watch;

pub async fn run_while_watched<B: CaptureBackend>(
    hub: &Hub,
    backend: &B,
    target: &CaptureTarget,
    viewers: &mut watch::Receiver<usize>,
    requests: &mut watch::Receiver<ViewerRequest>,
) -> WatchOutcome {
    tokio::select! {
        result = backend.run(hub, target) => WatchOutcome::Finished(result),
        () = viewers_gone_for(viewers, IDLE_GRACE) => WatchOutcome::Idle,
        () = request_changed(requests) => WatchOutcome::Changed,
    }
}
