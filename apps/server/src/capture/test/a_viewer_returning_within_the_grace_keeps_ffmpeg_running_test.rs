use super::{FakeBackend, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_viewer_returning_within_the_grace_keeps_ffmpeg_running() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let first = hub.join();

    settle().await;
    drop(first);
    sleep(Duration::from_secs(3)).await;

    let _second = hub.join();

    sleep(Duration::from_secs(30)).await;

    assert_eq!(backend.starts(), 1);
    assert_eq!(backend.kills(), 0);

    supervisor.abort();
}
