use super::{FakeBackend, FakeRun, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn viewers_leaving_during_the_restart_delay_stop_the_restart() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![FakeRun::EndsAfter {
        delay: Duration::from_millis(100),
        frames: 10,
    }]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let viewer = hub.join();

    settle().await;
    sleep(Duration::from_millis(400)).await;
    drop(viewer);
    sleep(Duration::from_secs(30)).await;

    assert_eq!(backend.starts(), 1);

    let _again = hub.join();

    settle().await;

    assert_eq!(backend.starts(), 2);

    supervisor.abort();
}
