use crate::capture::{
    CaptureBackend, CaptureTarget, IDLE_GRACE, WatchOutcome, request_changed, serve_screen_scale,
    viewers_gone_for,
};
use crate::display::ScreenScaler;
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use tokio::sync::watch;

pub async fn run_while_watched<B: CaptureBackend, S: ScreenScaler>(
    hub: &Arc<Hub>,
    backend: &B,
    scaler: &S,
    pipeline: &B::Pipeline,
    target: &CaptureTarget,
    viewers: &mut watch::Receiver<usize>,
    requests: &mut watch::Receiver<ViewerRequest>,
) -> WatchOutcome {
    tokio::select! {
        result = backend.run(hub, pipeline, target) => WatchOutcome::Finished(result),
        () = viewers_gone_for(viewers, IDLE_GRACE) => WatchOutcome::Idle,
        () = request_changed(requests) => WatchOutcome::Changed,
        never = serve_screen_scale(hub, target.output, scaler) => match never {},
    }
}
