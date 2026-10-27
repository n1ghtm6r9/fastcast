use crate::support::{
    TestViewer, a_bare_hello, hub_without_config, next_notice, start_endpoint, wait_for_viewers,
};
use app::display::DisplayScale;
use wire::SESSION_TIMEOUT;

#[test]
fn a_session_tells_its_viewer_each_scale_the_screen_shows() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 1);
    hub.show_scale(Some(DisplayScale {
        percent: 175,
        min: 100,
        max: 300,
    }));

    assert_eq!(
        next_notice(&viewer).as_deref(),
        Some(r#"{"type":"scale","percent":175,"min":100,"max":300}"#)
    );

    hub.show_scale(Some(DisplayScale {
        percent: 200,
        min: 100,
        max: 300,
    }));

    assert_eq!(
        next_notice(&viewer).as_deref(),
        Some(r#"{"type":"scale","percent":200,"min":100,"max":300}"#)
    );
}
