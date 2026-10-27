use super::{FakeScaler, next_shown_scale, scale_levels_of};
use crate::capture::serve_screen_scale;
use crate::hub::Hub;
use std::sync::Arc;

#[tokio::test]
async fn the_screen_scale_is_shown_once_the_service_runs() {
    let hub = Arc::new(Hub::new());
    let scaler = FakeScaler::new(2, Some(scale_levels_of(-2, 0, 4)), false);
    let mut shown = hub.watch_scale();
    let service = tokio::spawn({
        let hub = hub.clone();
        let scaler = scaler.clone();

        async move { serve_screen_scale(&hub, 2, scaler.as_ref()).await }
    });

    let scale = next_shown_scale(&mut shown).await;

    assert_eq!(scale.map(|scale| scale.percent), Some(150));

    service.abort();
}
