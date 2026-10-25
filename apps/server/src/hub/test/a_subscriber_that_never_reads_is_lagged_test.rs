use super::frame_timestamp;
use crate::fixtures::hub_with_config;
use crate::hub::BROADCAST_CAPACITY;
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn a_subscriber_that_never_reads_is_lagged() {
    let hub = hub_with_config();
    let mut subscription = hub.join();

    for index in 0..BROADCAST_CAPACITY * 2 {
        hub.publish(index == 0, index as u32, &[1]);
    }

    let Err(TryRecvError::Lagged(skipped)) = subscription.frames.try_recv() else {
        panic!("a subscriber that never reads falls behind");
    };
    let oldest_kept = subscription.frames.try_recv().expect("delivery resumes");

    assert!(skipped > 0);
    assert_eq!(u64::from(frame_timestamp(&oldest_kept)), skipped);
}
