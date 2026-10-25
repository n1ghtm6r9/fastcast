use super::another_request;
use crate::fixtures::{config_of, hub_with_config, x264_1920x1080_avcc};
use crate::hub::ViewerRequest;

#[test]
fn several_requests_before_the_reset_keep_the_old_stream_out() {
    let hub = hub_with_config();

    hub.publish(true, 0, &[1]);

    assert!(hub.request(another_request()));
    assert!(hub.request(ViewerRequest::default()));

    hub.set_config(config_of(&x264_1920x1080_avcc(), 60));
    hub.publish(true, 1, &[2]);

    let subscription = hub.join();

    assert!(subscription.config.is_none());
    assert!(subscription.gop.is_empty());
}
