use super::FakeBackend;
use crate::capture::{PrepareOutcome, prepare_while_watched};
use crate::hub::{Hub, ViewerRequest};

#[tokio::test(start_paused = true)]
async fn a_preparation_that_succeeds_is_reported_ready() {
    let hub = Hub::new();
    let _viewer = hub.join();
    let backend = FakeBackend::scripted(vec![]);
    let mut viewers = hub.watch_viewers();
    let mut requests = hub.watch_requests();

    let outcome = prepare_while_watched(
        backend.as_ref(),
        &ViewerRequest::default(),
        &mut viewers,
        &mut requests,
    )
    .await;

    assert!(matches!(outcome, PrepareOutcome::Ready(())));
}
