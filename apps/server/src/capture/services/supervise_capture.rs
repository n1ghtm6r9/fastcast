use crate::capture::{
    CaptureBackend, RESTART_DELAY, RETRY_DELAY_AFTER_FAILURE, WatchOutcome, run_while_watched,
};
use crate::hub::Hub;
use std::sync::Arc;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub async fn supervise_capture<B: CaptureBackend>(hub: Arc<Hub>, backend: Arc<B>) {
    let mut viewers = hub.watch_viewers();

    loop {
        let watched = viewers.wait_for(|count| *count > 0).await.is_ok();

        if !watched {
            return;
        }

        info!("a viewer is watching, starting the capture");

        let outcome = run_while_watched(&hub, backend.as_ref(), &mut viewers).await;

        hub.reset();

        match outcome {
            WatchOutcome::Idle => info!("no viewers left, the capture is stopped"),
            WatchOutcome::Finished(Ok(report)) => {
                warn!("ffmpeg stopped after {} frames, restarting", report.frames);
                sleep(RESTART_DELAY).await;
            }
            WatchOutcome::Finished(Err(failure)) => {
                error!("the capture failed: {failure}");
                sleep(RETRY_DELAY_AFTER_FAILURE).await;
            }
        }
    }
}
