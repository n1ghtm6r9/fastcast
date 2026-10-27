use super::{FakeScaler, next_shown_scale, scale_levels_of};
use crate::capture::serve_screen_scale;
use crate::hub::Hub;
use std::sync::Arc;

#[tokio::test]
async fn a_scale_ask_reaches_the_screen_and_its_new_scale_is_shown() {
    let hub = Arc::new(Hub::new());
    let scaler = FakeScaler::new(2, Some(scale_levels_of(-2, 0, 4)), false);
    let mut shown = hub.watch_scale();
    let service = tokio::spawn({
        let hub = hub.clone();
        let scaler = scaler.clone();

        async move { serve_screen_scale(&hub, 2, scaler.as_ref()).await }
    });

    next_shown_scale(&mut shown).await;
    hub.ask_scale(1);
    let scale = next_shown_scale(&mut shown).await;

    assert_eq!(scale.map(|scale| scale.percent), Some(175));
    assert_eq!(scaler.changes(), vec![(2, 1)]);

    service.abort();
}
