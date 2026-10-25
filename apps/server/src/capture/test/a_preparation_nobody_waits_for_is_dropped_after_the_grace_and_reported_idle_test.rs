use super::{FakeBackend, FakePreparation};
use crate::capture::{PrepareOutcome, prepare_while_watched};
use crate::hub::{Hub, ViewerRequest};
use std::time::Duration;
use tokio::time::Instant;

#[tokio::test(start_paused = true)]
async fn a_preparation_nobody_waits_for_is_dropped_after_the_grace_and_reported_idle() {
    let hub = Hub::new();
    let backend = FakeBackend::preparing(
        vec![FakePreparation::Takes {
            delay: Duration::from_secs(60),
        }],
        vec![],
    );
    let mut viewers = hub.watch_viewers();
    let mut requests = hub.watch_requests();
    let started = Instant::now();

    let outcome = prepare_while_watched(
        backend.as_ref(),
        &ViewerRequest::default(),
        &mut viewers,
        &mut requests,
    )
    .await;

    assert!(matches!(outcome, PrepareOutcome::Idle));
    let elapsed = started.elapsed();

    assert!(elapsed >= Duration::from_secs(5));
    assert!(elapsed < Duration::from_millis(5_100));
}
