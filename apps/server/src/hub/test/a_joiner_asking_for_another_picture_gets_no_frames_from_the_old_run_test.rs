use super::another_request;
use crate::fixtures::{config_of, hub_with_config, x264_1920x1080_avcc};
use tokio::sync::broadcast::error::TryRecvError;

#[test]
fn a_joiner_asking_for_another_picture_gets_no_frames_from_the_old_run() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);
    hub.request(another_request());

    let mut subscription = hub.join();

    hub.set_config(config_of(&x264_1920x1080_avcc(), 60));
    hub.publish(true, 1, &[2]);
    hub.publish(false, 2, &[3]);

    let latecomer = hub.join();

    assert!(matches!(
        subscription.frames.try_recv(),
        Err(TryRecvError::Empty)
    ));
    assert!(latecomer.config.is_none());
    assert!(latecomer.gop.is_empty());
}
