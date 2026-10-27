use super::{FakeBackend, FakeScaler, capture_target, scale_levels_of, settle};
use crate::capture::supervise_capture;
use crate::hub::Hub;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn the_capture_shows_the_scale_of_the_screen_it_runs_on() {
    let hub = Arc::new(Hub::new());
    let backend = FakeBackend::scripted(vec![]);
    let scaler = FakeScaler::new(
        capture_target().output,
        Some(scale_levels_of(-2, 1, 4)),
        false,
    );
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend.clone(), scaler));

    let _viewer = hub.join();
    settle().await;

    assert_eq!(
        hub.watch_scale().borrow().map(|scale| scale.percent),
        Some(175)
    );

    supervisor.abort();
}
