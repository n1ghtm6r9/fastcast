use super::{FakeBackend, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn ffmpeg_is_stopped_five_seconds_after_the_last_viewer_leaves() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let viewer = hub.join();

    settle().await;
    drop(viewer);
    sleep(Duration::from_millis(4_900)).await;

    assert_eq!(backend.kills(), 0);

    sleep(Duration::from_millis(200)).await;

    assert_eq!(backend.kills(), 1);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
