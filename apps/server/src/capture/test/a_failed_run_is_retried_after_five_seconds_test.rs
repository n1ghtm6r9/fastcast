use super::{FakeBackend, FakeRun, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_failed_run_is_retried_after_five_seconds() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![FakeRun::Fails {
        delay: Duration::from_millis(100),
    }]);
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone()));
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_millis(5_000)).await;

    assert_eq!(backend.starts(), 1);

    sleep(Duration::from_millis(200)).await;

    assert_eq!(backend.starts(), 2);

    supervisor.abort();
}
