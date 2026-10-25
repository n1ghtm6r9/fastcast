use crate::capture::viewers_gone_for;
use crate::hub::Hub;
use std::time::Duration;
use tokio::time::Instant;

#[tokio::test(start_paused = true)]
async fn viewers_gone_for_returns_after_the_grace_when_nobody_is_watching() {
    let hub = Hub::new();
    let mut viewers = hub.watch_viewers();
    let started = Instant::now();

    viewers_gone_for(&mut viewers, Duration::from_secs(5)).await;

    let elapsed = started.elapsed();

    assert!(elapsed >= Duration::from_secs(5));
    assert!(elapsed < Duration::from_millis(5_100));
}
