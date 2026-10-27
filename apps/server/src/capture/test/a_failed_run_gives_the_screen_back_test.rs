use super::{FakeBackend, FakeRun, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_failed_run_gives_the_screen_back() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![FakeRun::Fails {
        delay: Duration::from_millis(100),
    }]);
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_millis(5_200)).await;

    assert_eq!(backend.starts(), 2);
    assert_eq!(backend.opened().len(), 2);

    supervisor.abort();
}
