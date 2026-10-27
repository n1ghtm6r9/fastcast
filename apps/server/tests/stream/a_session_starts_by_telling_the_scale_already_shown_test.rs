use crate::support::{TestViewer, a_bare_hello, hub_without_config, next_notice, start_endpoint};
use app::display::DisplayScale;
use wire::SESSION_TIMEOUT;

#[test]
fn a_session_starts_by_telling_the_scale_already_shown() {
    let hub = hub_without_config();

    hub.show_scale(Some(DisplayScale {
        percent: 150,
        min: 100,
        max: 250,
    }));

    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    assert_eq!(
        next_notice(&viewer).as_deref(),
        Some(r#"{"type":"scale","percent":150,"min":100,"max":250}"#)
    );
}
