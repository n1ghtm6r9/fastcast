use super::{another_request, timestamps_of};
use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::Hub;

#[test]
fn a_joiner_asking_for_the_picture_already_playing_gets_the_snapshot_at_once() {
    let hub = Hub::new();

    assert!(hub.request(another_request()));

    hub.set_config(config_of(&nvenc_2560x1440_avcc(), 60));
    hub.publish(true, 0, &[1]);
    hub.publish(false, 1, &[2]);

    assert!(!hub.request(another_request()));

    let subscription = hub.join();

    assert!(subscription.config.is_some());
    assert_eq!(timestamps_of(&subscription.gop), vec![0, 1]);
}
