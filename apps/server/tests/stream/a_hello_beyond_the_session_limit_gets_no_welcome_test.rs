use crate::support::{
    REFUSAL_LIMIT, a_bare_hello, bind_peer, connect_viewers, hub_without_config, receive_within,
    send_hello, start_endpoint, wait_for_viewers,
};
use app::stream::MAX_SESSIONS;
use wire::SESSION_TIMEOUT;

#[test]
fn a_hello_beyond_the_session_limit_gets_no_welcome() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let _viewers = connect_viewers(endpoint.local_addr(), MAX_SESSIONS);

    wait_for_viewers(&hub, MAX_SESSIONS);

    let latecomer = bind_peer();

    send_hello(&latecomer, endpoint.local_addr(), &a_bare_hello());

    assert!(receive_within(&latecomer, REFUSAL_LIMIT).is_none());
    assert_eq!(hub.viewer_count(), MAX_SESSIONS);
}
