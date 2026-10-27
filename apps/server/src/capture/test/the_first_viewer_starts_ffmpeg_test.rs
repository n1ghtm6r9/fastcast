use super::{FakeBackend, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn the_first_viewer_starts_ffmpeg() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let _viewer = hub.join();

    settle().await;

    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
