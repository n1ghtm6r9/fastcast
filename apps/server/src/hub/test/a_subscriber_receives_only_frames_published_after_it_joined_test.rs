use super::{frame_timestamp, timestamps_of};
use crate::fixtures::hub_with_config;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn a_subscriber_receives_only_frames_published_after_it_joined() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    let mut subscription = hub.join();

    hub.publish(false, 1, &[2]);

    assert_eq!(timestamps_of(&subscription.gop), vec![0]);

    let live = subscription
        .frames
        .try_recv()
        .expect("the later frame is delivered");

    assert_eq!(frame_timestamp(&live), 1);
    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
}
