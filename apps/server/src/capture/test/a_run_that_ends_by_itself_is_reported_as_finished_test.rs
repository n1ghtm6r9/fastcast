use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{WatchOutcome, run_while_watched};
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn a_run_that_ends_by_itself_is_reported_as_finished() {
    let hub = Arc::new(Hub::new());
    let _viewer = hub.join();
    let backend = FakeBackend::scripted(vec![FakeRun::EndsAfter {
        delay: Duration::from_millis(100),
        frames: 10,
    }]);
    let mut viewers = hub.watch_viewers();

    let mut requests = hub.watch_requests();

    let outcome = run_while_watched(
        &hub,
        backend.as_ref(),
        &(),
        &capture_target(),
        &mut viewers,
        &mut requests,
    )
    .await;

    assert!(matches!(outcome, WatchOutcome::Finished(Ok(report)) if report.frames == 10));
}
