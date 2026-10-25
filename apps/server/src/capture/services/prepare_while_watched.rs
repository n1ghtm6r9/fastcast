use crate::capture::{
    CaptureBackend, IDLE_GRACE, PrepareOutcome, request_changed, viewers_gone_for,
};
use crate::hub::ViewerRequest;
use tokio::sync::watch;

pub async fn prepare_while_watched<B: CaptureBackend>(
    backend: &B,
    request: &ViewerRequest,
    viewers: &mut watch::Receiver<usize>,
    requests: &mut watch::Receiver<ViewerRequest>,
) -> PrepareOutcome<B::Pipeline> {
    tokio::select! {
        result = backend.prepare(request) => match result {
            Ok(pipeline) => PrepareOutcome::Ready(pipeline),
            Err(failure) => PrepareOutcome::Failed(failure),
        },
        () = viewers_gone_for(viewers, IDLE_GRACE) => PrepareOutcome::Idle,
        () = request_changed(requests) => PrepareOutcome::Changed,
    }
}
