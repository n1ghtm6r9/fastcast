use crate::capture::viewers_gone_for;
use crate::hub::Hub;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test(start_paused = true)]
async fn viewers_gone_for_never_returns_while_someone_is_watching() {
    let hub = Hub::new();
    let _viewer = hub.join();
    let mut viewers = hub.watch_viewers();

    let outcome = timeout(
        Duration::from_secs(600),
        viewers_gone_for(&mut viewers, Duration::from_secs(5)),
    )
    .await;

    assert!(outcome.is_err());
}
