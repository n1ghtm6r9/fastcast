use crate::support::{
    hub_with_config, second_viewer_is_served_after_the_first_vanished, start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn the_endpoint_keeps_serving_after_a_viewer_vanished() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);

    second_viewer_is_served_after_the_first_vanished(&endpoint, &hub);
}
