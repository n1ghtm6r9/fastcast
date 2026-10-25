use crate::fixtures::hub_with_config;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn reset_clears_the_config_and_the_gop() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);
    hub.reset();

    let mut subscription = hub.join();

    assert!(subscription.config.is_none());
    assert!(subscription.gop.is_empty());

    hub.publish(true, 1, &[2]);

    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
}
