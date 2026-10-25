use super::{another_request, frame_timestamp, timestamps_of};
use crate::fixtures::{config_of, hub_with_config, x264_1920x1080_avcc};

#[test]
fn a_joiner_asking_for_another_picture_gets_the_new_stream_once_it_starts() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);
    hub.request(another_request());

    let mut subscription = hub.join();

    hub.reset();

    assert!(hub.set_config(config_of(&x264_1920x1080_avcc(), 60)));

    hub.publish(true, 10, &[2]);

    let first = subscription
        .frames
        .try_recv()
        .expect("the new stream reaches the joiner");
    let latecomer = hub.join();

    assert_eq!(frame_timestamp(&first), 10);
    assert_eq!((first.config.width, first.config.height), (1920, 1080));
    assert!(latecomer.config.is_some());
    assert_eq!(timestamps_of(&latecomer.gop), vec![10]);
}
