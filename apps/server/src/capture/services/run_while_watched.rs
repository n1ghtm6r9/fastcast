use crate::capture::{CaptureBackend, IDLE_GRACE, WatchOutcome, viewers_gone_for};
use crate::hub::Hub;
use tokio::sync::watch;

pub async fn run_while_watched<B: CaptureBackend>(
    hub: &Hub,
    backend: &B,
    viewers: &mut watch::Receiver<usize>,
) -> WatchOutcome {
    tokio::select! {
        result = backend.run(hub) => WatchOutcome::Finished(result),
        () = viewers_gone_for(viewers, IDLE_GRACE) => WatchOutcome::Idle,
    }
}
