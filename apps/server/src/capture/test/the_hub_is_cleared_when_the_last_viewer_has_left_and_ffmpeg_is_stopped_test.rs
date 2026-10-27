use super::{FakeBackend, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn the_hub_is_cleared_when_the_last_viewer_has_left_and_ffmpeg_is_stopped() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let viewer = hub.join();

    settle().await;
    drop(viewer);
    sleep(Duration::from_secs(6)).await;

    assert!(hub.join().config.is_none());

    supervisor.abort();
}
