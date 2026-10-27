use super::{next_notice, start_session};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;

#[test]
fn a_session_tells_the_tv_to_diagnose_itself_when_the_hub_asks() {
    let hub = Arc::new(Hub::new());

    hub.diagnose(1_730_000_000_000);

    let session = start_session(&hub, ViewerRequest::default());

    assert_eq!(
        next_notice(&session.peer),
        r#"{"type":"diagnose","run":1730000000000}"#
    );

    session.stop_and_join();
}
