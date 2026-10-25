use super::{another_request, timestamps_of};
use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::Hub;

#[test]
fn a_request_made_before_any_config_does_not_hold_back_the_first_stream() {
    let hub = Hub::new();

    assert!(hub.request(another_request()));
    assert!(hub.set_config(config_of(&nvenc_2560x1440_avcc(), 60)));

    hub.publish(true, 0, &[1]);

    let subscription = hub.join();

    assert!(subscription.config.is_some());
    assert_eq!(timestamps_of(&subscription.gop), vec![0]);
}
