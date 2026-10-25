use crate::hub::Hub;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn frames_published_before_any_config_are_dropped() {
    let hub = Hub::new();
    let mut subscription = hub.join();

    hub.publish(true, 0, &[1]);

    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
    assert!(hub.join().gop.is_empty());
}
