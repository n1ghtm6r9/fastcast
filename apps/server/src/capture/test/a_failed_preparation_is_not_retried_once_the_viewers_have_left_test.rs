use super::{FakeBackend, FakePreparation, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_failed_preparation_is_not_retried_once_the_viewers_have_left() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(vec![FakePreparation::Fails], vec![]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let viewer = hub.join();

    settle().await;
    drop(viewer);
    sleep(Duration::from_secs(30)).await;

    assert_eq!(backend.prepares(), 1);
    assert_eq!(backend.starts(), 0);

    let _again = hub.join();

    settle().await;

    assert_eq!(backend.prepares(), 2);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
