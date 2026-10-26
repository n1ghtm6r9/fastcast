use super::{FakeBackend, capture_target};
use crate::capture::{WatchOutcome, run_while_watched};
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

#[tokio::test(start_paused = true)]
async fn a_run_nobody_watches_is_stopped_after_the_grace_and_reported_idle() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let mut viewers = hub.watch_viewers();
    let started = Instant::now();

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

    assert!(matches!(outcome, WatchOutcome::Idle));
    let elapsed = started.elapsed();

    assert!(elapsed >= Duration::from_secs(5));
    assert!(elapsed < Duration::from_millis(5_100));
    assert_eq!(backend.kills(), 1);
}
