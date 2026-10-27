use super::{FakeBackend, FakePreparation, FakeRun, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn a_kept_screen_is_given_back_when_the_next_preparation_fails() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::preparing(
        vec![FakePreparation::Succeeds, FakePreparation::Fails],
        vec![FakeRun::EndsAfter {
            delay: Duration::from_millis(100),
            frames: 10,
        }],
    );
    let supervisor = tokio::spawn(supervise_capture(
        hub.clone(),
        backend.clone(),
        FakeScaler::unknown(),
    ));
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_secs(7)).await;

    assert_eq!(backend.prepares(), 3);
    assert_eq!(backend.starts(), 2);
    assert_eq!(backend.opened().len(), 2);

    supervisor.abort();
}
