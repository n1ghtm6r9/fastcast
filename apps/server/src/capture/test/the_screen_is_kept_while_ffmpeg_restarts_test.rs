use super::{FakeBackend, FakeRun, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn the_screen_is_kept_while_ffmpeg_restarts() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![FakeRun::EndsAfter {
        delay: Duration::from_millis(100),
        frames: 10,
    }]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_millis(1_200)).await;

    assert_eq!(backend.starts(), 2);
    assert_eq!(backend.opened().len(), 1);

    supervisor.abort();
}
