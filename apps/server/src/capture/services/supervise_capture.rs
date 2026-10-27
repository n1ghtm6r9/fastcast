use crate::capture::{
    CaptureBackend, PrepareOutcome, RESTART_DELAY, WatchOutcome, prepare_while_watched,
    release_screen, run_while_watched, wait_before_retry,
};
use crate::display::ScreenScaler;
use crate::hub::Hub;
use std::sync::Arc;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub async fn supervise_capture<B: CaptureBackend, S: ScreenScaler>(
    hub: Arc<Hub>,
    backend: Arc<B>,
    scaler: Arc<S>,
) {
    let mut viewers = hub.watch_viewers();
    let mut requests = hub.watch_requests();
    let mut kept = None;

    loop {
        if *viewers.borrow() == 0 {
            release_screen(kept.take()).await;
        }

        let watched = viewers.wait_for(|count| *count > 0).await.is_ok();

        if !watched {
            release_screen(kept.take()).await;
            return;
        }

        info!("a viewer is watching, starting the capture");

        let request = requests.borrow_and_update().clone();

        let prepared =
            prepare_while_watched(backend.as_ref(), &request, &mut viewers, &mut requests).await;

        let pipeline = match prepared {
            PrepareOutcome::Ready(pipeline) => pipeline,
            PrepareOutcome::Failed(failure) => {
                error!("the capture could not be prepared: {failure}");
                release_screen(kept.take()).await;
                wait_before_retry(&mut requests).await;
                continue;
            }
            PrepareOutcome::Idle => {
                info!("no viewers left, the capture is not started");
                release_screen(kept.take()).await;
                continue;
            }
            PrepareOutcome::Changed => {
                info!("a viewer asked for another picture, preparing the capture again");
                continue;
            }
        };

        let screen = match kept.take() {
            Some(screen) if screen.request.mode == request.mode => screen,
            stale => {
                release_screen(stale).await;
                backend.open_screen(&request).await
            }
        };

        let outcome = run_while_watched(
            &hub,
            backend.as_ref(),
            scaler.as_ref(),
            &pipeline,
            &screen.target,
            &mut viewers,
            &mut requests,
        )
        .await;

        hub.reset();

        match outcome {
            WatchOutcome::Idle => {
                info!("no viewers left, the capture is stopped");
                release_screen(Some(screen)).await;
            }
            WatchOutcome::Changed => {
                info!("a viewer asked for another picture, restarting the capture");
                kept = Some(screen);
            }
            WatchOutcome::Finished(Ok(report)) => {
                warn!(
                    "the capture stopped after {} frames, restarting",
                    report.frames
                );
                kept = Some(screen);
                sleep(RESTART_DELAY).await;
            }
            WatchOutcome::Finished(Err(failure)) => {
                error!("the capture failed: {failure}");
                release_screen(Some(screen)).await;
                wait_before_retry(&mut requests).await;
            }
        }
    }
}
