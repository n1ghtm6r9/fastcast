use crate::support::{
    LONG_SESSION_TIMEOUT, hub_without_config, next_tv_event, start_endpoint, tv_link,
    wait_for_viewers,
};
use app::display::DisplayScale;
use fastcast_native::LinkEvent;

#[test]
fn a_scale_from_the_server_reaches_the_tv_link() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), LONG_SESSION_TIMEOUT);
    let link = tv_link(endpoint.local_addr());

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);
    wait_for_viewers(&hub, 1);
    hub.show_scale(Some(DisplayScale {
        percent: 125,
        min: 100,
        max: 250,
    }));

    assert_eq!(
        next_tv_event(&link),
        LinkEvent::Notice(r#"{"type":"scale","percent":125,"min":100,"max":250}"#.to_owned())
    );
}
