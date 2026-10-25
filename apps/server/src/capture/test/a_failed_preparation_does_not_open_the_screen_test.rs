use super::{FakeBackend, FakePreparation, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn a_failed_preparation_does_not_open_the_screen() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(vec![FakePreparation::Fails], vec![]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let _viewer = hub.join();

    settle().await;

    assert_eq!(backend.prepares(), 1);
    assert!(backend.opened().is_empty());
    assert_eq!(backend.starts(), 0);

    supervisor.abort();
}
