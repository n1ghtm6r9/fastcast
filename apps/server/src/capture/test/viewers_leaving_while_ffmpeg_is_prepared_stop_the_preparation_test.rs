use super::{FakeBackend, FakePreparation, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn viewers_leaving_while_ffmpeg_is_prepared_stop_the_preparation() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(
        vec![FakePreparation::Takes {
            delay: Duration::from_secs(60),
        }],
        vec![],
    );
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let viewer = hub.join();

    settle().await;
    drop(viewer);
    sleep(Duration::from_secs(30)).await;

    assert!(backend.opened().is_empty());
    assert_eq!(backend.starts(), 0);

    let _again = hub.join();

    settle().await;

    assert_eq!(backend.prepares(), 2);
    assert_eq!(backend.opened().len(), 1);
    assert_eq!(backend.starts(), 1);

    supervisor.abort();
}
