use super::{FakeBackend, FakePreparation, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_failed_preparation_is_retried_after_five_seconds() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(vec![FakePreparation::Fails], vec![]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_millis(4_900)).await;

    assert_eq!(backend.prepares(), 1);

    sleep(Duration::from_millis(200)).await;

    assert_eq!(backend.prepares(), 2);
    assert_eq!(backend.opened().len(), 1);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
