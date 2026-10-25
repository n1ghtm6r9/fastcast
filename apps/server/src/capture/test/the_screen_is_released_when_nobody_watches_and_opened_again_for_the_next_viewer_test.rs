use super::{FakeBackend, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn the_screen_is_released_when_nobody_watches_and_opened_again_for_the_next_viewer() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let first = hub.join();

    settle().await;
    drop(first);
    sleep(Duration::from_secs(6)).await;

    let _second = hub.join();

    settle().await;

    assert_eq!(backend.opened().len(), 2);
    assert_eq!(backend.starts(), 2);

    supervisor.abort();
}
