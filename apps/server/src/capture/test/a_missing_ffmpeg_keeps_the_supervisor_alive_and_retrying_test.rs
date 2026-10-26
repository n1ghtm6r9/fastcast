use super::default_settings;
use crate::capture::{FfmpegBackend, locate_ffmpeg, supervise_capture};
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_missing_ffmpeg_keeps_the_supervisor_alive_and_retrying() {
    let hub = Arc::new(Hub::new());
    let backend = Arc::new(FfmpegBackend::with_locator(default_settings(), || {
        locate_ffmpeg(&[])
    }));
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend));
    let _viewer = hub.join();

    sleep(Duration::from_secs(12)).await;

    assert!(!supervisor.is_finished());

    supervisor.abort();
}
