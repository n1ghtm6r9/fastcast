use crate::capture::viewers_gone_for;
use crate::hub::Hub;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{Instant, sleep};

#[tokio::test(start_paused = true)]
async fn viewers_gone_for_starts_the_grace_over_when_a_viewer_comes_and_goes() {
    let hub = Arc::new(Hub::new());
    let mut viewers = hub.watch_viewers();
    let visitor = hub.clone();

    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;

        let viewer = visitor.join();

        sleep(Duration::from_secs(1)).await;
        drop(viewer);
    });

    let started = Instant::now();

    viewers_gone_for(&mut viewers, Duration::from_secs(5)).await;

    let elapsed = started.elapsed();

    assert!(elapsed >= Duration::from_secs(9));
    assert!(elapsed < Duration::from_millis(9_100));
}
