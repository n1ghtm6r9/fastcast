use crate::support::{
    ARRIVAL_LIMIT, TestViewer, a_bare_hello, hub_with_config, next_config, start_endpoint,
    wait_until,
};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_viewer_report_reaches_the_hub() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let reports = hub.watch_reports();

    next_config(&mut viewer);
    viewer.send(&Packet::Stats("{\"decoded\":300,\"dropped\":0}"));

    assert!(wait_until(ARRIVAL_LIMIT, || {
        reports.borrow().as_deref() == Some("{\"decoded\":300,\"dropped\":0}")
    }));
}
