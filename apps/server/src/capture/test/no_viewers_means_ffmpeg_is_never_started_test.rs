use super::FakeBackend;
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn no_viewers_means_ffmpeg_is_never_started() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));

    sleep(Duration::from_secs(60)).await;

    assert_eq!(backend.starts(), 0);

    supervisor.abort();
}
