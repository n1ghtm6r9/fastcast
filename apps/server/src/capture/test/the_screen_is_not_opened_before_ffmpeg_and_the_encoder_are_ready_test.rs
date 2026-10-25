use super::{FakeBackend, FakePreparation, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn the_screen_is_not_opened_before_ffmpeg_and_the_encoder_are_ready() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(
        vec![FakePreparation::Takes {
            delay: Duration::from_secs(3),
        }],
        vec![],
    );
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_secs(1)).await;

    assert_eq!(backend.prepares(), 1);
    assert!(backend.opened().is_empty());
    assert_eq!(backend.starts(), 0);

    sleep(Duration::from_secs(3)).await;

    assert_eq!(backend.opened().len(), 1);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
