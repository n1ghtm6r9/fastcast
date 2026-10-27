use super::{FakeBackend, FakeRun, FakeScaler, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test(start_paused = true)]
async fn ffmpeg_that_exits_while_viewers_remain_is_restarted_after_one_second() {
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
    let _viewer = hub.join();

    settle().await;
    sleep(Duration::from_millis(1_000)).await;

    assert_eq!(backend.starts(), 1);

    sleep(Duration::from_millis(200)).await;

    assert_eq!(backend.starts(), 2);

    supervisor.abort();
}
